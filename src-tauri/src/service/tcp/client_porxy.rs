use std::{collections::HashMap, error::Error, sync::Mutex};
use tokio::sync::mpsc::Sender;

use tokio::sync::mpsc;

use crate::service::sender::Client;

type Connection = Mutex<HashMap<i64, Sender<String>>>;

pub struct ClientProxy {
    connections: Connection,
    from_id: i64,
}

impl ClientProxy {
    pub fn new(from_id: i64) -> Self {
        Self {
            connections: Mutex::new(HashMap::new()),
            from_id,
        }
    }
}

impl ClientProxy {
    pub async fn send_msg(
        &self,
        to_id: i64,
        pay_load: Option<&str>,
    ) -> Result<(), Box<dyn Error + '_>> {
        if pay_load.is_none() {
            return Ok(());
        }
        let conn;
        let pay_load = pay_load.unwrap().to_string();
        {
            let mut connections = self.connections.try_lock()?;
            match connections.get(&to_id) {
                Some(connection) => {
                    conn = connection.clone();
                }
                None => {
                    let (sender, receiver) = mpsc::channel::<String>(100);
                    let from_id = self.from_id;
                    tokio::spawn(async move {
                        let mut client = Client::new(from_id, to_id, receiver);
                        client.run().await;
                    });
                    conn = sender.clone();
                    connections.insert(to_id, sender);
                }
            }
        }
        conn.send(pay_load).await?;
        Ok(())
    }
}

// async fn send_msg<'a>(
//     user_id: i64,
//     pay_load: Option<&str>,
//     db: Db,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let a = DB.lock().unwrap();
//     while DB.lock().await.get(&user_id).is_none() {
//         let mut stream = TcpStream::connect(format!("{}:{}", "192.168.31.171", "8086")).await?;
//         let id_struct = IdStruct {
//             id: user_id,
//             ip: "127.0.0.1".into(),
//             port: "8086".into(),
//         };
//         let clien_req = ClientReq {
//             aim_user: user_id,
//             client: id_struct,
//         };
//         let data = serde_json::to_vec(&clien_req).unwrap();
//         stream.write_all(&data).await?;
//         stream.shutdown().await?;
//         let mut buffer = Vec::new();
//         stream.read_to_end(&mut buffer).await?;
//         if let Some(resp) = serde_json::from_slice::<ClientResp>(&buffer)?.aim_user {
//             db.lock().await.insert(user_id, DbData { id_data: resp });
//         }
//         sleep(time::Duration::from_millis(100));
//     }
//     info!("{:?}", pay_load);
//     let db_data = db.lock().await.get(&user_id).unwrap().clone();
//     let ip = &db_data.id_data.ip;
//     let port = &db_data.id_data.port;
//     let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
//     stream
//         .write_all(pay_load.unwrap_or_default().as_bytes())
//         .await?;
//     Ok(())
// }
