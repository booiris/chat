use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::sync::Mutex;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::model::DbData;
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
    pub async fn send_msg(
        &self,
        user_id: i64,
        pay_load: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        let mut stream = TcpStream::connect(format!("192.168.31.130:{}", SERVER_PORT)).await?;
        let pay_load: Payload = serde_json::from_str(pay_load.unwrap_or_default())?;
        stream.write_all(pay_load.message.as_bytes()).await?;
        Ok(())
    }
}
