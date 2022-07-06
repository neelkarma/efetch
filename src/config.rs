use crate::errors::Error;
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub excuse: ExcuseConfig,
    #[serde(default)]
    pub colors: ColorsConfig,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        if let Ok(raw_config) = fs::read_to_string(path) {
            if let Ok(config) = toml::from_str::<Config>(&raw_config) {
                Ok(config)
            } else {
                Err(Error::ConfigParseError)
            }
        } else {
            Err(Error::ConfigReadError)
        }
    }

    pub fn read() -> Self {
        // TODO: Find a better way to structure this
        if let Ok(path) = get_config_path() {
            if let Ok(config) = Self::from_path(&path) {
                config
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ExcuseConfig {
    pub disabled: bool,
}

impl Default for ExcuseConfig {
    fn default() -> Self {
        Self { disabled: false }
    }
}

#[derive(Deserialize, Debug)]
pub struct ColorsConfig {
    pub disabled: bool,
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self { disabled: false }
    }
}

fn get_config_path() -> Result<PathBuf, Error> {
    if let Ok(mut path) = env::var("HOME") {
        path.push_str("/.config/excuses/config.toml");
        Ok(PathBuf::from(path))
    } else {
        Err(Error::NoHomeEnvError)
    }
}
