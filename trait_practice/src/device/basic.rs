pub trait Device<T> {
    fn device_handler(&self, event: T);
    fn processor(&self);
}

pub struct CMOCLW002W<T> {
    pub event_type: T,
}

impl Device<Event> for CMOCLW002W<Event> {
    fn device_handler(&self, event: Event) {
        match event {
            Event::SingleClick(1) => {
                println!("single click 1 !")
            }
            Event::SingleClick(2) => {
                println!("single click 2!")
            }
            Event::DoubleClick(1) => {
                println!("double click 1 !")
            }
            _ => {
                println!("other event :{:?}", event)
            }
        }
    }
    fn processor(&self) {
        println!("processor")
    }
}

#[derive(Debug)]
pub enum Event {
    SingleClick(u8),
    DoubleClick(u8),
}
