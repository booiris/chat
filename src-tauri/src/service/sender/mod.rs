use local_ip_address::local_ip;
use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::Receiver,
};

use log::{debug, error, info};
use tokio::net::TcpStream;

use crate::{
    consts::{PROXY_IP, PROXY_PORT, SERVER_PORT},
    model::{ClientReq, ClientResp, IdStruct},
};

pub struct Client {
    from_id: i64,
    to_id: i64,
    client_data: Option<ClientData>,
    receiver: Receiver<String>,
}

struct ClientData {
    ip: String,
    port: String,
}

impl Client {
    pub fn new(from_id: i64, to_id: i64, receiver: Receiver<String>) -> Self {
        debug!("new client from id: {} to id: {}", from_id, to_id);
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
        debug!("connect: ({} : {}) exit", self.from_id, self.to_id);
    }

    async fn handle_data(&mut self, data: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.client_data.is_none() {
            self.get_stream().await?;
        }
        let client_data = self.client_data.as_mut().unwrap();
        let mut stream =
            TcpStream::connect(format!("{}:{}", client_data.ip, client_data.port)).await?;
        stream.write_all(data.as_bytes()).await?;
        stream.shutdown().await?;
        Ok(())
    }

    async fn get_stream(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            let mut stream = TcpStream::connect(format!("{}:{}", PROXY_IP, PROXY_PORT)).await?;
            let my_local_ip = local_ip().unwrap().to_string();
            info!("{}", my_local_ip);
            let id_struct = IdStruct {
                id: self.from_id,
                ip: my_local_ip,
                port: SERVER_PORT.into(),
            };
            let clien_req = ClientReq {
                aim_user: self.to_id,
                client: id_struct,
            };
            let data = serde_json::to_vec(&clien_req).unwrap();
            stream.write_all(&data).await?;
            stream.shutdown().await?;
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).await?;
            if let Some(resp) = serde_json::from_slice::<ClientResp>(&buffer)?.aim_user {
                debug!("build connect success! ip: {} port: {}", resp.ip, resp.port);
                self.client_data = Some(ClientData {
                    ip: resp.ip,
                    port: resp.port,
                });
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Ok(())
    }
}
