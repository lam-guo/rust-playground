use inotify::{EventMask, Inotify, WatchMask};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let r = watch_file(PathBuf::from("/home/watch"));
    println!("result:{:?}", r);
}

fn watch_file(path: PathBuf) -> std::io::Result<()> {
    let mut buffer = [0u8; 4096];
    let mut inotify = Inotify::init()?;

    inotify.add_watch(&path, WatchMask::MODIFY)?;

    loop {
        let events = inotify.read_events_blocking(&mut buffer)?;

        for event in events {
            if event.mask.contains(EventMask::MODIFY) {
                if let Some(name) = event.name {
                    if let Some(str_name) = name.to_str() {
                        if str_name == "test.txt" {
                            let mut file = File::open(&PathBuf::from("/home/watch/test.txt"))?;
                            let mut buffer = String::new();
                            file.read_to_string(&mut buffer)?;
                            let s = buffer.trim();
                            println!("File contents changed:\n{:?}", s);
                        }
                    }
                }
            }
            match event.mask {
                // 处理不同事件类型（例如创建、修改、删除）
                _ => println!("event: {:?}", event),
            }
        }
    }
}
