use serde::Deserialize;

use crate::proxy::ProxyType;
use crate::rule::CompiledRule;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_bind")]
    pub bind: String,

    #[serde(default, rename = "log-level")]
    pub log_level: LogLevel,

    #[serde(default)]
    pub proxies: Option<Vec<ProxyConfig>>,
}

impl Config {
    pub fn new(config: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_yaml::from_str(config)?)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

fn default_port() -> u16 {
    1999
}

fn default_bind() -> String {
    "127.0.0.1".to_string()
}

#[derive(Deserialize)]
pub struct ProxyConfig {
    pub name: String,

    #[serde(flatten)]
    pub r#type: ProxyType,

    pub rules: Option<Vec<CompiledRule>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = r#"
port: 1999
bind: '*'

log-level: 'debug'

proxies:
  - name: 'proxy'
    type: 'http'
    host: 'localhost'
    port: 8080
    rules:
      - 'ip.src == 127.0.0.1'

  - name: 'direct'
    type: 'direct'
    inner: something
"#;
        let config: Config = serde_yaml::from_str(config).unwrap();
        assert_eq!(config.port, 1999);
        assert_eq!(&config.bind, "*");
        assert_eq!(config.log_level, LogLevel::Debug);
        // assert_eq!(config.proxies.unwrap()[0].inner, "");
    }

    #[test]
    fn test_parse_missing_optionals() {
        let config = r#""#;
        let config: Config = serde_yaml::from_str(config).unwrap();
        assert_eq!(config.port, 1999);
        assert_eq!(&config.bind, "127.0.0.1");
        assert_eq!(config.log_level, LogLevel::Info);
    }
}
