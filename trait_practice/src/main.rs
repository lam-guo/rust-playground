use serde::{Serialize, Deserialize};

fn main() {
    let s =Switch::from((1,1));
    println!("{:?}",s);
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
    fn from((channel,status_int): (u8, u8)) -> Self {
        Self {
            channel,
            status: match status_int {
                0 => "off".to_string(),
                _ => "on".to_string(),
            },
        }
    }
}