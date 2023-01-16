fn main() {
    closure_fn(|| {
        // 这里只会warn，不会执行，path statement with no effect(无效声明)
        closure;
    });
    closure_fn(|| {
        closure();
    });
    closure_fn(closure); // 思考与第5行区别是什么？

    closure_fn(|| closure2(1_u8));
    //下面代码会报错,对应第8行代码
    //expected a `FnOnce<()>` closure, found `()`
    //the trait `FnOnce<()>` is not implemented for `()`
    //wrap the `()` in a closure with no arguments: `|| { /* code */ }`
    //clo(closure2(1_u8))
}

fn closure() {
    println!("go!");
}

fn closure2(param: u8) {
    println!("go!param is :{:?}", param);
}

fn closure_fn<F: FnOnce()>(closure_fn: F) {
    closure_fn();
}
