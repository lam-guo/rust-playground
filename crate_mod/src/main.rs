pub mod gg {
    pub fn say() {
        println!("gg");
    }
}

mod lam;

fn main() {
    gg::say();
    lam::basic::say();
    println!("Hello, world!");
}
