use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server_address: String,
    pub server_port: u16,
    pub node : String,
    pub password : String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut conf = Config::new();

        // read the local file only in development mode
        #[cfg(debug_assertions)]
        conf.merge(File::with_name("assets/channel-config.ini"))?;

        #[cfg(not(debug_assertions))]
        conf.merge(File::with_name("/usr/local/etc/channel-config.ini"))?;

        conf.try_into()
    }
}