use std::ops::Add;
use std::time::Duration;
use tokio::time::{self, Instant};

#[tokio::main]
async fn main() {
    // timer().await
    race().await
}

async fn race() {
    tokio::select! {
        _=a()=>{}
        _=b()=>{}
    }
}

async fn timer() {
    let mut next_record_at = Instant::now().add(Duration::from_secs(20));
    loop {
        tokio::select! {
            _ = time::sleep_until(next_record_at) => {
                println!("inside sleep_until------------------------,{:?}",next_record_at);
                next_record_at = Instant::now().add(Duration::from_secs(20));
            }
            _ = a() => {
                next_record_at = Instant::now().add(Duration::from_secs(10));
                println!("inside a------------------------");
            }
            // 下面代码就会看到进入sleep_until逻辑了
            // _ = a() => {
            //     next_record_at = Instant::now().add(Duration::from_secs(4));
            //     println!("inside a------------------------");
            // }
        }
    }
}
 
async fn a() {
    println!("into a------------------------");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("out a------------------------");
}

// b比a慢，只会输出into b，不会输出out b
// TODO 了解下为什么out b不输出？
async fn b() {
    println!("into b------------------------");
    tokio::time::sleep(Duration::from_secs(6)).await;
    println!("out b------------------------");
}
