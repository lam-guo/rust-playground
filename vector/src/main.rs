fn main() {
    println!("Hello, world!");
    let mut b: Vec<u8> = vec![];
    for i in 0..10 {
        b.push(i);
    }
    println!("{:?}", b);
    // 这样不行，会提示does not have a constant size known at compile-time。详见https://stackoverflow.com/questions/49393462/what-does-str-does-not-have-a-constant-size-known-at-compile-time-mean-and
    // println!("{:?}", b[2..4]);
    println!("{:?}", &b[2..4]);
}
