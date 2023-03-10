use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Duration;

use tokio::fs::File;
use tokio::process::Command;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

const WIFI_STATUS_WAIT: u8 = 0;
const WIFI_STATUS_RECONNECT: u8 = 1;
const WIFI_STATUS_SUCCEED: u8 = 2;

const WPA_CLI_STATUS_OK: u8 = 1;

static WIFI: Lazy<Mutex<Wifi>> = Lazy::new(|| {
    Mutex::new(Wifi {
        status: WIFI_STATUS_WAIT,
    })
});

pub struct Wifi {
    pub status: u8,
}

// 1.启动检查wifi配置
//  1.1不存在->待配网（开启ap，状态设为WIFI_STATUS_WAIT）
//    1.1.1 待配网10分钟无操作，进入重连模式 ->（1.2）
//    1.1.2 配网成功，进入WIFI模式 ->（2）
//    1.1.3 配网失败，进入重连模式 ->（1.2）
//  1.2存在->重连模式，尝试连接网络（开启sta，状态为WIFI_STATUS_RECONNECT）
//    1.2.1 重连成功,进入WIFI模式 ->（2）
//    1.2.2 重连失败，自旋 ->（1.2）
//  2.WIFI模式（状态设为WIFI_STATUS_SUCCEED)
//    2.1 如果使用中途，网络异常断开，回去重连模式 ->（1.2）
pub async fn start() {
    if let Ok(result) = check_conf().await {
        match result {
            true => sta_start().await,
            false => ap_start().await,
        }
    }
}

// 开启ap模式
// 1.调用sta禁用脚本
// 2.调用ap启动脚本
pub async fn ap_start() {
    kill_sta().await;
    cmd_ap_start().await;
    // TODO
    // 1.状态改为 WIFI_STATUS_WAIT
    // 2.启动http server监听，接收账号密码配置
    //  2.1 10分钟无响应 -> 3
    //  2.2 接收到配置 -> 3
    // 3.server close
    // 4.sta_start()
    let mut wifi = WIFI.lock().unwrap();
    wifi.status = WIFI_STATUS_WAIT;
    drop(wifi);
    let listener = TcpListener::bind("0.0.0.0:8888").await.unwrap();

    loop {
        tokio::select! {
            _=tokio::time::sleep(Duration::from_secs(60*10))=>{
                break;
            }
             a = listener.accept() => {
                if let Ok((stream,_)) = a {
                   if handle_connection(stream).await {
                        break;
                   }
                }
            }
        }
    }
    sta_start().await;
}

// 开启sta模式
// 1.调用ap禁用脚本
// 2.调用sta启动脚本
pub async fn sta_start() {
    kill_ap().await;
    cmd_wpa_supplicant().await;
    // TODO
    // 1.状态改为 WIFI_STATUS_RECONNECT
    // 2.判断wifi状态 (无限循环)
    //   2.1 连接失败，continue
    //   2.2 连接成功，状态改为 WIFI_SUCCEED, continue
    //   2.3 接收到中断信号（break，ap_start())
    let mut wifi = WIFI.lock().unwrap();
    wifi.status = WIFI_STATUS_RECONNECT;
    drop(wifi);
    loop {
        tokio::select! {
            _=tokio::time::sleep(Duration::from_secs(5))=>{
                let status = wifi_status().await;
                let mut wifi = WIFI.lock().unwrap();
                wifi.status = WIFI_STATUS_RECONNECT;
                if status == WPA_CLI_STATUS_OK {
                    wifi.status = WIFI_STATUS_SUCCEED;
                }
                drop(wifi);
            }
            // TODO 接收中断信号
        }
    }
}

// 检查配置是否存在
async fn check_conf() -> std::io::Result<bool> {
    let mut f = File::open("/etc/wpa_supplicant.conf").await?;
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer).await?;
    Ok(true)
}

// 处理请求
// 1.处理接收wifi配置请求
// 2.检查wifi配置合法性？
// 3.wifi配置写入配置文件 （TODO 考虑多个wifi？）
async fn handle_connection(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    // TODO 处理请求
    let request = String::from_utf8_lossy(&buffer[..]);
    return false;
}

// 启动ifconfig
pub async fn ifconfig_up() {
    let output = Command::new("ifconfig")
        .args(["wlan0", "up"])
        .output()
        .await
        .expect("failed to execute process");
    println!("{:?}", output);
}

// TODO 错误处理？
pub async fn cmd_ap_start() {
    ifconfig_up().await;
    hostapd().await;
    udhcpd().await;
}

// 启动hostapd
pub async fn hostapd() {
    let output = Command::new("hostapd")
        .args(["-B", "/etc/hostapd.conf"])
        .output()
        .await
        .expect("failed to exec hostapd");
    println!("{:?}", output);
}

// 启动udhcpd
pub async fn udhcpd() {
    let output = Command::new("udhcpd")
        .output()
        .await
        .expect("failed to exec udhcpd");
    println!("{:?}", output);
}

// ifconfig配置
pub async fn cmd_ifconfig_ap() {
    let output = Command::new("ifconfig")
        .args(["wlan0", "192.168.2.1"])
        .output()
        .await
        .expect("failed to exec ap_ifconfig");
    println!("{:?}", output);
}

// 关闭ap模式
pub async fn kill_ap() {
    let output = Command::new("killall")
        .args(["hostapd", "udhcpd"])
        .output()
        .await
        .expect("failed to kill ap");
    println!("{:?}", output);
}

// TODO 错误处理
pub async fn cmd_wpa_supplicant() {
    ifconfig_up().await;
    wpa_supplicant().await;
    udhcpc().await;
}

// 启动wpa
pub async fn wpa_supplicant() {
    let output = Command::new("wpa_supplicant")
        .args(["-i", "wlan0", "-c", "/etc/wpa_supplicant.conf"])
        .output()
        .await
        .expect("failed to exec wpa_supplicant");
    println!("{:?}", output);
}

pub async fn udhcpc() {
    let output = Command::new("udhcpc")
        .output()
        .await
        .expect("failed to exec udhcpc");
    println!("{:?}", output);
}

pub async fn kill_sta() {
    let output = Command::new("killall")
        .args(["wpa_supplicant", "udhcpc"])
        .output()
        .await
        .expect("failed to kill sta");
    println!("{:?}", output);
}

pub async fn wifi_status() -> u8 {
    let output = Command::new("wpa_cli")
        .args(["status"])
        .output()
        .await
        .expect("failed to get wpa_cli status");
    println!("{:?}", output);
    return 0;
}
