use serde::Deserialize;
use crate::proxy::http::HttpProxy;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ProxyType {
    Http(HttpProxy),
    Direct,
}

