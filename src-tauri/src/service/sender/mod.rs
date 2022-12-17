use std::{
    error::Error,
    sync::{Arc, Condvar, Mutex},
};
use tokio::{sync::mpsc::Receiver, io::AsyncWriteExt};

use log::{debug, error};
use tokio::net::TcpStream;

pub struct Client {
    from_id: i64,
    to_id: i64,
    client_data: Option<ClientData>,
    receiver: Receiver<String>,
}

struct ClientData {
    ip: String,
    port: i32,
    stream: TcpStream,
}

impl Client {
    pub fn new(from_id: i64, to_id: i64, receiver: Receiver<String>) -> Self {
        debug!("new client from {} to {}", from_id, to_id);
        Self {
            from_id,
            to_id,
            client_data: None,
            receiver,
        }
    }

    pub async fn run(&mut self) {
        while let Some(data) = self.receiver.recv().await {
            if let Err(err) = self.handle_data(data).await {
                error!(
                    "connect: ({} : {}) handle data error! Err: {}",
                    self.from_id, self.to_id, err
                );
                break;
            }
        }
        debug!("connect: ({} : {}) exit", self.from_id, self.to_id)
    }

    async fn handle_data(&mut self, data: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.client_data.is_none() {
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            let mut stream = TcpStream::connect(format!("{}:{}", "192.168.31.171", "8086"));
        }
        self.client_data.as_mut().unwrap().stream.write_all(data.as_bytes()).await?;
        Ok(())
    }
}
