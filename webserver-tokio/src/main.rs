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

// TODO 对比std::thread::spawn与tokio::spawn的区别。优劣势
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();
    rt.spawn(async move {
        let srv = HttpServer::new(|| App::new().service(greet).service(svc::service::init()))
            .bind(("0.0.0.0", 9999))
            .expect("msg")
            .run();
        srv.await.unwrap();
    });
    rt.block_on(async {
        let _ = HttpServer::new(|| App::new().service(greet).service(svc::service::init()))
            .bind(("0.0.0.0", 8888))
            .unwrap()
            .run()
            .await;
    });
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
