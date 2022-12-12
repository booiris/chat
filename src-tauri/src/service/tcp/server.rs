use crate::consts::*;
use crate::model::Payload;
use log::{error, info};
use tauri::Window;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
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
                info!("new client: {:?}", addr);
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
    let mut buffer = Vec::new();
    let mut data = Vec::new();
    loop {
        match buf_reader.read_buf(&mut buffer).await {
            Err(err) => {
                error!("read from client error, err: {}", err);
                break;
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                data.append(&mut buffer);
            }
        }
    }
    window
        .emit(
            "get-msg",
            data.into_iter().map(|x| x as char).collect::<String>(),
        )
        .unwrap();
}
