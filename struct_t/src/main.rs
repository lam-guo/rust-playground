fn main() {
    // let s = [A::default();5]; // 这里会提示没有实现Copy
    // println!("{:?}",s)
    let b = [B::default(); 5];
    println!("{:?}", b)
}

#[derive(Default, Debug)] // 这里又因为String, 无法使用Copy.
struct A {
    s: String,
}

#[derive(Default, Debug, Copy, Clone)]
struct B {
    i: u8,
}
