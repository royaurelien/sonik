use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub source: String,
    pub target: String,
}

pub fn load() -> anyhow::Result<Config> {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::config_dir().unwrap());

    let path = base.join("sonik").join("config.toml");

    let raw = std::fs::read_to_string(&path)?;
    let conf: Config = toml::from_str(&raw)?;
    Ok(conf)
}
