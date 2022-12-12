use std::sync::mpsc::Receiver;

pub struct Client {
    ip: String,
    port: i32,
    from_id: i64,
    to_id: i64,
    receiver: Receiver<String>,
}

impl Client {
    pub fn new(
        ip: String,
        port: i32,
        from_id: i64,
        to_id: i64,
        receiver: Receiver<String>,
    ) -> Self {
        Self {
            ip,
            port,
            from_id,
            to_id,
            receiver,
        }
    }
}

impl Client {
    pub fn run(&self) {
        while let Ok(msg) = self.receiver.recv() {
            println!("msg: {}", msg);
        }
    }
}
