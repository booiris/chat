use crate::consts::*;
use crate::model::Payload;
use tauri::Window;
use tokio::io::AsyncBufReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub async fn server(window: Window) {
    let listener = TcpListener::bind(format!("{}:{}", IP, SERVER_PORT))
        .await
        .unwrap();
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                let window = window.clone();
                println!("new client: {:?}", addr);
                tokio::spawn(async move {
                    process(socket, window).await;
                });
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

async fn process(socket: TcpStream, window: Window) {
    let (reader, _writer) = socket.into_split();
    tokio::spawn(read_from_client(reader, window));
}

async fn read_from_client(reader: OwnedReadHalf, window: Window) {
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut buf = String::new();
    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(_e) => {
                eprintln!("read from client error");
                break;
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                // let message = buf.drain(..).as_str().to_string();
                // window.emit("get-msg", Payload { message }).unwrap();
            }
        }
    }
    let (msg_tx, msg_rx) = mpsc::channel::<String>(100);
}
