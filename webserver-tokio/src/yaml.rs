use anyhow::anyhow;
use notify::{Error, Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;

pub fn load_yaml_file<T>(path: impl AsRef<Path>) -> anyhow::Result<T>
where
    T: Default + Serialize + DeserializeOwned,
{
    let default_config = T::default();
    if !Path::new(path.as_ref()).exists() {
        let mut file = File::create(path.as_ref())?;
        file.write_all(serde_yaml::to_string(&default_config)?.as_bytes())?;
    }
    let file = File::open(path.as_ref())?;
    let config: T = serde_yaml::from_reader(file).map_err(|e| anyhow!("read yaml error {}", e))?;

    Ok(config)
}

pub fn write_yaml_file<T>(path: impl AsRef<Path>, config: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    let buf = serde_yaml::to_string(config)?;
    // info!("file buf: {}", buf);
    file.write(buf.as_bytes())?;
    Ok(())
}

pub fn load_and_watch_config_file<T>(path: &str) -> (INotifyWatcher, Receiver<T>)
where
    T: Default + Serialize + DeserializeOwned + std::marker::Sync + std::marker::Send + 'static,
{
    let path = String::from(path);
    let path_clone = path.clone();
    let config = load_yaml_file(&path).unwrap();
    let (tx, rx) = watch::channel(config);
    let mut watcher = RecommendedWatcher::new(
        move |result: Result<Event, Error>| {
            if let Ok(event) = result {
                if event.kind.is_modify() {
                    match load_yaml_file(&path_clone) {
                        Ok(new_config) => {
                            tx.send_if_modified(|cfg: &mut T| {
                                *cfg = new_config;
                                true
                            });
                        }
                        Err(_) => {}
                    }
                }
            }
        },
        notify::Config::default(),
    )
    .unwrap();
    let p = Path::new(&path);
    watcher.watch(p, RecursiveMode::NonRecursive);

    (watcher, rx)
}
