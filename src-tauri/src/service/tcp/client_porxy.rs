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