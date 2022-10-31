use serde::{Deserialize, Serialize};

use crate::device::basic::Device;
mod device;

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
