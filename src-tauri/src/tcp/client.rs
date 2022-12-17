use std::error::Error;

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{consts::SERVER_PORT, model::Payload};

pub async fn send_msg(pay_load: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("192.168.31.130:{}", SERVER_PORT)).await?;
    let pay_load: Payload = serde_json::from_str(pay_load.unwrap_or_default())?;
    stream.write_all(pay_load.message.as_bytes()).await?;
    Ok(())
}
