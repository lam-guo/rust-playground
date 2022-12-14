fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let a: u8 = 10;
    // 这里会overflow，处理方法参考checked,wrapping,saturating;
    // 参考文章：https://kangxiaoning.github.io/post/2021/02/rust-integer-overflow-handled/

    // println!("{:?}", 5 - a);

    println!("{:?}", 5_i32.checked_sub(a as i32));
    println!("{:?}", 5_i32.saturating_sub(a as i32));
    println!("{:?}", 5_i32.wrapping_sub(a as i32));
}
