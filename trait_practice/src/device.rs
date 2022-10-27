pub trait Device {
    fn mqtt_handler(&mut self);
    fn device_handler(&mut self);
    fn processor(&mut self);
}

pub struct Switch {}

impl Device for Switch{
    fn mqtt_handler(&mut self) {
        println!("mqtt!")
    }
    fn device_handler(&mut self) {
        println!("device!")
    }
    fn processor(&mut self) {
        println!("processor!")
    }
}
