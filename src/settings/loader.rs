use std::{env::{self, VarError}};
use config::{Config, ConfigError, Environment, File};
use log::{debug, info};
use super::types::Settings;

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let res_cfg_file = env::var("MSG_PRODUCER_CONFIG_FILE")
            .and_then(|val| {
                match val.as_str() {
                    "" => Err(VarError::NotPresent),
                    _other => Ok(val)
                }
            });
        let cfg_filename;
        if res_cfg_file.is_err(){
            cfg_filename = "config.yml".to_string();
        } else {
            cfg_filename = res_cfg_file.unwrap();
        }
        info!("Loading settings from file {}", &cfg_filename);
        let s = Config::default()
            .merge(File::with_name(&cfg_filename)).expect("Failed to load settings from file")
            .merge(Environment::with_prefix("MSG_PRODUCER").separator("_")).unwrap()
            .clone();

        debug!("Loaded settings: {:?}", &s);
        s.try_into()
    }
}