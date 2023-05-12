use anyhow::{anyhow, Result};
use nix::libc::{c_int, pid_t};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::Command;

fn main() {
    let result = get_pid_by_name("signal");
    println!("result:{:?}", result);
    std::thread::sleep(std::time::Duration::from_secs(5));
    let rs = system_call(Signal::SIGKILL, "signal");
    // 已经kill掉了，下面逻辑按理说不会执行
    println!("result:{:?}", rs);
    std::thread::sleep(std::time::Duration::from_secs(10));
}

pub fn system_call(sig: Signal, p_name: &str) -> Result<()> {
    let pid = get_pid_by_name(p_name);
    if let None = pid {
        return Err(anyhow!("p_name:{} not exist", p_name));
    }
    match kill(Pid::from_raw(pid.unwrap()), sig) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!("Error sending signal: {}", err)),
    }
}

fn get_pid_by_name(name: &str) -> Option<i32> {
    let output = Command::new("pgrep").arg("-o").arg(name).output().ok()?;

    if output.status.success() {
        let pid = String::from_utf8(output.stdout).ok()?.trim().parse().ok()?;
        Some(pid)
    } else {
        None
    }
}
