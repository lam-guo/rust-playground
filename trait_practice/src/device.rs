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

pub struct Light {}

impl Device for Light{
    fn mqtt_handler(&mut self) {
        println!("mqtt:light!")
    }
    fn device_handler(&mut self) {
        println!("device:light!")
    }
    fn processor(&mut self) {
        println!("processor!:light")
    }
}
