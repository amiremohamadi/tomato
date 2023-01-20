use crate::proxy::http::HttpProxy;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ProxyType {
    Http(HttpProxy),
    Direct,
}
