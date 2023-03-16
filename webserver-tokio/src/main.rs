use crate::cmd::command::ls;
use actix_web::{get, web, App, HttpServer, Responder};
use std::fs;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod cmd;
mod svc;

// #[tokio::main]
// async fn main() {
//     ls().await;
//     let listener = TcpListener::bind("0.0.0.0:8888").await.unwrap();
//     loop {
//         let (stream, _) = listener.accept().await.unwrap();
//         handle_connection(stream).await;
//     }
// }

// async fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).await.unwrap();

//     let contents = fs::read_to_string("index.html").unwrap();

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );

//     stream.write_all(response.as_bytes()).await.unwrap();
// }

#[tokio::main]
async fn main() {
    let _ = HttpServer::new(|| App::new().service(greet).service(svc::service::init()))
        .bind(("0.0.0.0", 8888))
        .expect("msg")
        .run()
        .await;
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
