use core::time;
use log::{debug, info};
use std::thread::sleep;
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::model::{ClientReq, ClientResp, DbData, IdStruct};
use crate::{consts::SERVER_PORT, model::Payload};

type Db = Arc<Mutex<HashMap<i64, Arc<DbData>>>>;

pub struct Client {
    db: Db,
}

impl Client {
    pub fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Client {
    pub async fn send_msg(&self, user_id: i64, pay_load: Option<&str>) {
        let db = self.db.clone();
        send_msg(user_id, pay_load, db).await.expect("send msg err");
    }
}

async fn send_msg<'a>(
    user_id: i64,
    pay_load: Option<&str>,
    db: Db,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    while db.lock().await.get(&user_id).is_none() {
        let mut stream = TcpStream::connect(format!("{}:{}", "192.168.31.171", "8086")).await?;
        let id_struct = IdStruct {
            id: user_id,
            ip: "127.0.0.1".into(),
            port: "8086".into(),
        };
        let clien_req = ClientReq {
            aim_user: user_id,
            client: id_struct,
        };
        let data = serde_json::to_vec(&clien_req).unwrap();
        stream.write_all(&data).await?;
        stream.shutdown().await?;
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;
        if let Some(resp) = serde_json::from_slice::<ClientResp>(&buffer)?.aim_user {
            db.lock()
                .await
                .insert(user_id, Arc::new(DbData { id_data: resp }));
        }
        sleep(time::Duration::from_millis(100));
    }
    info!("{:?}", pay_load);
    let db_data = db.lock().await.get(&user_id).unwrap().clone();
    let ip = &db_data.id_data.ip;
    let port = &db_data.id_data.port;
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
    stream
        .write_all(pay_load.unwrap_or_default().as_bytes())
        .await?;
    Ok(())
}
