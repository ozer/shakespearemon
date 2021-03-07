use serde::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Application {
    pub host: String,
    pub port: u16,
    pub poke_api_base_url: String,
    pub shakespeare_translator_api_base_url: String
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub application: Application
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config"))?;

        s.try_into()
    }
}