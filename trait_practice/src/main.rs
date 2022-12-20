use serde::{Deserialize, Serialize};

use crate::device::basic::Device;
mod device;

mod bound;

fn main() {
    let s = Switch::from((1, 1));
    println!("{:?}", s);
    let switch = device::device::Switch {};
    test_trait(switch);
    let light = device::device::Light {};
    test_trait(light);
    let b = device::basic::CMOCLW002W {
        event_type: device::basic::Event::SingleClick(1),
    };
    let event = device::basic::Event::SingleClick(1);
    b.device_handler(event);
    let event = device::basic::Event::DoubleClick(1);
    b.device_handler(event);
    let event = device::basic::Event::DoubleClick(3);
    b.device_handler(event);

    test_bound();
}

#[derive(Serialize, Deserialize, Debug)]
struct Switch {
    channel: u8,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResultCmdList {
    accept_cmdlist: Vec<String>,
    report_cmdlist: Vec<String>,
}

impl From<(u8, u8)> for Switch {
    fn from((channel, status_int): (u8, u8)) -> Self {
        Self {
            channel,
            status: match status_int {
                0 => "off".to_string(),
                _ => "on".to_string(),
            },
        }
    }
}

fn test_trait(mut c: impl device::device::Device) {
    c.processor();
    c.mqtt_handler();
    c.device_handler();
}

fn test_bound() {
    let data = vec!["a", "b"];
    bound::cache::save(data);

    bound::cache::save([1]);

    // 下面代码会报错，提示Sized没有实现
    // 编译器准确的告诉了我们原因：str 字符串切片它是 DST 动态大小类型，这意味着编译器无法在编译期知道 str 类型的大小，只有到了运行期才能动态获知，这对于强类型、强安全的 Rust 语言来说是不可接受的。
    // 引用：https://course.rs/difficulties/slice.html
    // let s: str = "banana";
    // bound::cache::save(s);
}
