use std::ops::Deref;

// 解引用
/*
    虽然实现get方法一样可以获取结构体的值。
    但是实现Deref trait的主要目的是为了提供灵活性，以便为不同类型实现类似指针的行为，从而在更广泛的上下文中使用它们
*/
fn main() {
    let a = MyType::<i32>(3);
    println!("{}", *a);
    let b = MyTypeG::<&str>("gg");
    println!("{}", b.get());
}

struct MyType<T>(T);

impl<T> Deref for MyType<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyTypeG<T>(T);

impl<T> MyTypeG<T> {
    fn get(&self) -> &T {
        &self.0
    }
}
