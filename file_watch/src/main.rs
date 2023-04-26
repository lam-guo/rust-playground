use anyhow::Result;
use chrono::Local;
use inotify::{EventMask, Inotify, WatchMask};
use log::*;
use std::f32::consts::E;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
use tokio;
use tokio::sync::mpsc::Sender;

const BUTTON_FILE: &str = "t.txt";
const LONG_PRESS_DURATION: i64 = 8;

#[derive(Debug)]
enum ButtonState {
    Up = 0,
    Down = 1,
}

#[tokio::main]
async fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();
    let (btn_rx, mut btn_tx) = tokio::sync::mpsc::channel::<ButtonState>(1);
    rt.spawn(async move {
        let mut last_up = Local::now().timestamp();
        let mut last_down = Local::now().timestamp();
        let mut ticker = tokio::time::interval(Duration::from_secs(1));
        let mut state = ButtonState::Up;
        let mut long_block = tokio::sync::Mutex::new(false);
        loop {
            tokio::select! {
                button_msg = btn_tx.recv() => {
                    if let Some(s) = button_msg {
                        state = s;
                        match state{
                            ButtonState::Up => {
                                last_up = Local::now().timestamp();
                                if last_up - last_down >LONG_PRESS_DURATION {
                                    //这就是长按
                                    let mut  block = long_block.lock().await;
                                    if !*block {
                                        println!("长按松开");
                                    }
                                    *block = false;
                                }else {
                                    //这是短按
                                }
                            },
                            ButtonState::Down => {
                                last_down = Local::now().timestamp();
                            },
                        }
                    }else {
                        println!("not in?")
                    }
                }
                _= ticker.tick() =>{
                    //判断下相隔时间，避免出现一直按住的情况
                    let mut block = long_block.lock().await;
                    if last_up - last_down > LONG_PRESS_DURATION && !*block {
                        //  这就是长按
                        *block = true;
                        println!("长按没松开");
                    }
                }

            }
        }
    });
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async move {
            let r = watch_file(PathBuf::from("/home/test/"), BUTTON_FILE, btn_rx.clone()).await;
            println!("watch_file result:{:?}", r);
        });
    });

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => {
            }
            _ = async { tokio::time::sleep(Duration::from_secs(5)).await } => {
                println!("slept for 5 seconds");
            }
        }
    }
}

async fn watch_file(
    dir_path: PathBuf,
    file_name: &str,
    rx: Sender<ButtonState>,
) -> std::io::Result<()> {
    let mut buffer = [0u8; 4096];
    let mut inotify = Inotify::init()?;

    inotify.add_watch(&dir_path, WatchMask::MODIFY)?;
    let mut file_path = dir_path.clone();
    file_path.push(file_name);
    println!(
        "dir:{:?},file_path:{:?}",
        dir_path.clone(),
        file_path.clone()
    );
    loop {
        let events = inotify.read_events_blocking(&mut buffer)?;
        for event in events {
            if event.mask == EventMask::MODIFY && event.name.is_some() {
                if let Some(str_name) = event.name.unwrap().to_str() {
                    if str_name == file_name {
                        let mut file = File::open(&file_path)?;
                        let mut buffer = String::new();
                        file.read_to_string(&mut buffer)?;
                        let content = buffer.trim();
                        if let Err(e) = match content {
                            "0" => {
                                println!("is 0");
                                rx.send(ButtonState::Up).await
                            }
                            _ => {
                                println!("is 1");
                                rx.send(ButtonState::Down).await
                            }
                        } {
                            println!("watch file rx send error:{:?}", e);
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
}
