use chrono::*;

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

    let mut job_list = JobList { list: vec![] };
    let j1 = Job {
        time: Local::now().timestamp(),
        func: || job(1_u8),
    };
    job_list.list.push(j1);
    let j2 = Job {
        time: Local::now().timestamp(),
        func: || job2("你很厉害啊".to_string()),
    };
    job_list.list.push(j2);
    job_list.run();
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

fn job(param: u8) {
    println!("job!param is :{:?}", param);
}
fn job2(param: String) {
    println!("job!param is :{:?}", param);
}

pub struct JobList {
    list: Vec<Job>,
}

pub struct Job {
    pub time: i64,
    pub func: fn(),
}

impl JobList {
    fn run(&self) {
        for i in self.list.iter() {
            if i.time <= Local::now().timestamp() {
                i.func.clone()();
            }
        }
    }
}
