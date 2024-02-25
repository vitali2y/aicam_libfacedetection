use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct General {
    pub device: String,
    pub confidence: u16,
    pub output_dir: String,
    pub debug: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub general: General,
}

impl Settings {
    pub fn new() -> Result<Self, toml::de::Error> {
        let cfg_file = format!("config/{}.toml", env!("CARGO_PKG_NAME"));
        toml::from_str(
            &fs::read_to_string(cfg_file.clone())
                .unwrap_or_else(|_| panic!("Config {cfg_file} file is absent")),
        )
    }
}
