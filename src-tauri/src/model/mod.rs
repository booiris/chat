#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Payload {
    pub message: String,
    pub time: time
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct IdStruct {
    pub id: i64,
    pub ip: String,
    pub port: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct DbData {
    pub id_data: IdStruct,
}
