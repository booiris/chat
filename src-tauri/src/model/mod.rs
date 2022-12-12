#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Payload {
    pub text: String,
    pub time: String,
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

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientReq {
    pub client: IdStruct,
    pub aim_user: i64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct ClientResp {
    pub aim_user: Option<IdStruct>,
}