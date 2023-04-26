use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub async fn ls() {
    let mut command = Command::new("ls");
    command.stdout(Stdio::piped());
    let mut child = command.spawn().unwrap();
    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");
    let mut reader = BufReader::new(stdout).lines();
    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    let mut output: String = "".into();
    while let Some(line) = reader.next_line().await.unwrap() {
        println!("Line: {}", line);
        output.push_str(&line);
    }
}
