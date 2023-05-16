use std::ops::Add;
use std::time::Duration;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let mut next_record_at = Instant::now().add(Duration::from_secs(20));
    loop {
        tokio::select! {
            _ = tokio::time::sleep_until(next_record_at) => {
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
    tokio::time::sleep(Duration::from_secs(5)).await;
}
