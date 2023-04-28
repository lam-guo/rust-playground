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
const OVER_LONG_PRESS_DURATION: i64 = 15;

#[derive(Debug, PartialEq, Eq)]
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
        let mut last_down = 0;
        let mut ticker = tokio::time::interval(Duration::from_secs(1));
        let long_press_block = tokio::sync::Mutex::new(false);
        let over_long_press_block = tokio::sync::Mutex::new(false);
        let mut cur_state = ButtonState::Up;
        loop {
            tokio::select! {
                button_msg = btn_tx.recv() => {
                    if let Some(state) = button_msg {
                        match state{
                            ButtonState::Up => {
                                cur_state = ButtonState::Up;
                                let now = Local::now().timestamp();
                                if now - last_down > LONG_PRESS_DURATION && now -last_down <OVER_LONG_PRESS_DURATION{
                                    let mut  block = long_press_block.lock().await;
                                    if !*block {
                                        // 这里做一层保障，避免tick错过判断
                                        // TODO 长按逻辑
                                        println!("长按-1");
                                    }
                                    *block = false;
                                }else if now - last_down > OVER_LONG_PRESS_DURATION {
                                    let mut  block = long_press_block.lock().await;
                                    *block = false;
                                    drop(block);
                                    let mut o_block = over_long_press_block.lock().await;
                                    if !*o_block {
                                        // 这里做一层保障，避免tick错过判断
                                        // TODO 超长按逻辑
                                        println!("超长按-1");
                                    }
                                    *o_block = false;
                                } else {
                                    // 短按
                                }
                            },
                            ButtonState::Down => {
                                last_down = Local::now().timestamp();
                                cur_state = ButtonState::Down;
                            },
                        }
                    }else {
                        info!("btn_tx recv None");
                    }
                }
                _= ticker.tick() =>{
                    //判断下相隔时间，避免出现一直按住的情况
                    if cur_state == ButtonState::Up{
                        continue;
                    }
                    let now = Local::now().timestamp();
                    if now -last_down > OVER_LONG_PRESS_DURATION {
                        let mut block = over_long_press_block.lock().await;
                        if *block {
                            continue;
                        }
                        *block = true;
                        // TODO 超长按逻辑
                        println!("超长按")
                    }
                    if now - last_down> LONG_PRESS_DURATION {
                        let mut block = long_press_block.lock().await;
                        if *block {
                            continue;
                        }
                        *block = true;
                        // TOOD长按逻辑
                        println!("长按");
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
                            "0" => rx.send(ButtonState::Up).await,
                            _ => rx.send(ButtonState::Down).await,
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
