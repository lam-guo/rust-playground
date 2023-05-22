fn main() {
    let v1 = vec![1, 2, 3];
    for val in v1.iter() {
        println!("Got: {}", val);
    }
    iter_map();
}

// 思考一下rust的map方法跟别的语言的map方法的区别（如js)
// 为什么map不支持在里面做额外的事情？

// 下面是一段map方法的描述，注意 “lazy”这个词
/// // don't do this:
/// (0..5).map(|x| println!("{x}"));
///
/// // it won't even execute, as it is lazy. Rust will warn you about this.

/* 粘一段chatGpt的回复
在 Rust 中，Iterator 的方法通常是惰性（lazy）的。这意味着当你调用一个 Iterator 方法时，它并不会立即执行，而是返回另一个 Iterator，该 Iterator 将对应的操作延迟到以后执行，
只有在你调用最终消耗（consuming）迭代器方法时才会执行。

例如，在你的代码示例中，map() 方法是一个惰性方法，它不会立即执行，而是返回一个新的 Iterator，该 Iterator 将在以后执行。
由于你没有调用任何最终消耗迭代器的方法，例如 for_each()、collect() 等，因此 map() 方法根本没有执行，输出语句也没有执行。

这种惰性的设计有助于提高程序的性能和灵活性。通过惰性地处理数据，可以避免不必要的计算和内存分配，从而提高程序的效率。此外，惰性的设计也使得 Iterator 方法可以链式调用，从而提高代码的可读性和可维护性。

然而，惰性的设计也有一些坑点，例如如果你忘记调用最终消耗迭代器的方法，可能会导致代码不执行或不符合预期。因此，在使用 Iterator 方法时，需要特别注意惰性和及早消耗（eager）的区别，以确保代码正确执行。
 */
fn iter_map() {
    let a: Vec<u8> = vec![1, 2, 3, 4];
    let mut total = 0;
    let mut b = a.iter().map(|x| {
        let c = x * 2;
        total += c;
        c
    });
    while b.next().is_some() {
        println!("{:?}", b.next(),);
    }
    // 这里输出20，是10的2倍
    println!("{}", total);
}
