#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Payload {
    pub message: String,
}
