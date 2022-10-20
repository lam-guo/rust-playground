fn main() {
    clo(|| {
        go; // 这里只会warn，不会执行，path statement with no effect(无效声明)
    });
    clo(|| {
        go();
    });
    clo(go); // 思考与第5行区别是什么？
}

fn go() {
    println!("go!");
}

fn clo<F: FnOnce()>(closure_fn: F) {
    closure_fn();
}
