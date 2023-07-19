use std::sync::mpsc::{channel, Receiver};

pub fn mpsc1<T>(value: T) -> Receiver<Event<T>>
where
    T: Send + Sync + 'static,
{
    let (rx, tx) = channel::<Event<T>>();
    std::thread::spawn(move || match rx.send(Event(Some(A { value }), None)) {
        Ok(r) => println!("send ok:{:?}", r),
        Err(e) => println!("send err:{}", e),
    });
    tx
}

#[derive(Debug, PartialEq)]
pub struct Event<T>(pub Option<A<T>>, pub Option<B>);

#[derive(Debug, PartialEq)]
pub struct A<T> {
    pub value: T,
}

#[derive(Debug, PartialEq)]
pub struct B {}
