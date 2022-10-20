use std::sync::Mutex;

fn main() {
    let b = Mutex::new(5);
    {
        let mut c = b.lock().unwrap();
        *c = 6;
    }
    println!("{:?}", b);
    dead_lock();
}

fn dead_lock() {
    let b = Mutex::new(5);
    let mut c = b.lock().unwrap();
    *c = 6;
    drop(c);
    let mut m = b.lock().unwrap();
    *m = 7;
    println!("{:?}", m);
    // drop(m); //不注释可以打印b的值，不drop会导致b的值被locked，打印出：Mutex { data: <locked>, poisoned: false, .. }
    println!("{:?}", b);
}
