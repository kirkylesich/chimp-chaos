#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub promql_base: Option<String>,
    pub tempo_base: Option<String>,
    pub http_bind: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { promql_base: None, tempo_base: None, http_bind: Some("0.0.0.0:8080".to_string()) }
    }
}

pub fn load() -> Settings {
    let mut builder = config::Config::builder();
    builder = builder
        .add_source(config::Environment::with_prefix("CHAOS").separator("__"));
    let cfg = builder.build();
    let mut s = match cfg {
        Ok(c) => c.try_deserialize().unwrap_or_default(),
        Err(_) => Settings::default(),
    };
    if s.http_bind.is_none() {
        s.http_bind = Some("0.0.0.0:8080".to_string());
    }
    s
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(load);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_bind() {
        let s = load();
        assert!(s.http_bind.is_some());
    }
}

