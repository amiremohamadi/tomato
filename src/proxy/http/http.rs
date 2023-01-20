use serde::Deserialize;

#[derive(Deserialize)]
pub struct HttpProxy {
    pub host: String,
    pub port: u16,
}
