use tera::{Context, Tera};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Child, Stdio};
use tokio::io::BufReader as bf;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;

#[tokio::main]
async fn main() {
    read();
    read_wpa();
    async_read_wpa().await;
}

fn read() {
    match Tera::new("./templates/*.conf") {
        Ok(t) => {
            let mut context = Context::new();
            context.insert("ssid", "wifi_test");
            context.insert("password", "123456");
            match File::create("wpa.conf") {
                Ok(f) => {
                    let r = t.render_to("wpa.conf", &context, f);
                    println!("{:?}", r);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Parsing error(s): {}", e);
        }
    };
}

fn read_wpa() {
    match File::open("wpa.conf") {
        Ok(f) => {
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let l = line.unwrap();
                if l.contains("ssid=") {
                    let v: Vec<_> = l.split("=").collect();
                    for i in v {
                        println!("{:?}", i);
                    }
                }
            }
        }
        Err(e) => {
            println!("open error:{:?}", e)
        }
    }
}

pub async fn async_read_wpa() {
    let mut command = Command::new("cat");
    command.arg("wpa.conf");
    command.stdout(Stdio::piped());
    let mut child = command.spawn().unwrap();
    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");
    let mut reader = bf::new(stdout).lines();
    while let Some(l) = reader.next_line().await.unwrap() {
        if l.contains("ssid=") {
            let v: Vec<_> = l.split("=").collect();
            for i in v {
                println!("{:?}", i);
            }
        }
    }
}
