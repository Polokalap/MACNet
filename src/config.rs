use std::fs;
use std::process::exit;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::logger::{error, info};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    key: String,
}

impl Config {
    pub(crate) fn key(&self) -> String {
        self.key.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            key: Uuid::new_v4().to_string(),
        }
    }
}

pub async fn init() -> Config {

    info("Initializing config...").await;

    match fs::read_to_string("config.toml") {
        Ok(contents) => match toml::from_str(&contents) {
            Ok(config) => {

                info("Config initialized!").await;
                config

            },
            Err(err) => {
                error("Malformed config!").await;
                exit(1);
            }
        },
        Err(_) => {
            let default_config = Config::default();
            if let Ok(toml_str) = toml::to_string(&default_config) {
                let _ = fs::write("config.toml", toml_str);
            }
            info("Config initialized!").await;
            default_config
        }
    }

}