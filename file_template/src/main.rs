use tera::{Context, Tera};

use std::fs::File;

fn main() {
    read()
}

fn read() {
    let tera = match Tera::new("./templates/*.conf") {
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
