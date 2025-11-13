/// config.rs
/// Application configuration structures and loader.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::fmt;

use crate::utils::slug::unique_slug;

pub const DEFAULT_CONFIG: &str = include_str!("../assets/default_config.yaml");


/// Top-level configuration
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub device: Vec<DeviceConfig>,
    pub watch: WatchConfig,
}

/// Watcher configuration
#[derive(Debug, Deserialize, Clone)]
pub struct WatchConfig {
    pub enabled: bool,
    pub debounce_ms: u64,
}

/// Device entry in YAML
#[derive(Debug, Deserialize, Clone)]
pub struct DeviceConfig {
    pub name: String,
    pub label: Option<String>,
    pub mount: String,
    pub mountinfo: bool,
    pub folders: Vec<FolderConfig>,
}

/// Folder (source to target) entry in YAML
#[derive(Debug, Deserialize, Clone)]
pub struct FolderConfig {
    pub source: String,
    pub target: String,
    pub enabled: bool,
}

/// Runtime structure used for each sync operation
#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub device_name: String,
    pub source: PathBuf,
    pub target: PathBuf,
    pub index_path: PathBuf,
    pub enabled: bool,
}


/// Get path to config file
pub fn get_config_file() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("sonik")
        .join("config.yaml")
}


/// Create config file + parent dirs if missing
fn bootstrap_config(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(path, DEFAULT_CONFIG)
        .with_context(|| format!("Failed to write default config to {}", path.display()))?;

    Ok(())
}

/// Load configuration from file, bootstrapping if missing
pub fn load_config() -> Result<AppConfig> {
    let path = get_config_file();

    if !path.exists() {
        bootstrap_config(&path)?;
    }

    let txt = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config: {}", path.display()))?;

    let conf: AppConfig = serde_yaml::from_str(&txt)
        .with_context(|| format!("Invalid YAML in {}", path.display()))?;

    Ok(conf)
}

impl AppConfig {
    /// Build a list of all required sync configurations
    pub fn build_sync_configs(&self) -> Result<Vec<SyncConfig>> {
        self._build_sync_configs(true)
    }

    pub fn _build_sync_configs(&self, active_only: bool) -> Result<Vec<SyncConfig>> {
        let mut out = Vec::new();

        for device in &self.device {
            let device_root= device.expanded_path();            

            for folder in &device.folders {
                if active_only && !folder.enabled {
                    continue;
                }
                let index_path = dirs::data_dir()
                    .unwrap()
                    .join("sonik")
                    .join(&device.name)
                    .join(format!("{}.bin", unique_slug(&folder.target, &device.name)));

                let target_path = device_root.join(&folder.target);

                out.push(SyncConfig {
                    device_name: device.name.clone(),
                    source: PathBuf::from(&folder.source),
                    target: target_path,
                    index_path,
                    enabled: folder.enabled,
                });
            }
        }

        Ok(out)
    }

}

impl fmt::Display for SyncConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} | {}] {} to {} (index: {})",
            self.device_name,
            if self.enabled { "enabled" } else { "disabled" },
            self.source.display(),
            self.target.display(),
            self.index_path.display(),
        )
    }
}

impl DeviceConfig {
    /// Expand patterns like {uid}, {user} and {device}
    pub fn expanded_path(&self) -> PathBuf {
        let uid = nix::unistd::getuid().as_raw();
        let user = whoami::username();

        let expanded = self.mount
            .replace("{uid}", &uid.to_string())
            .replace("{user}", &user)
            .replace("{device}", &self.name);

        // tracing::info!("Expanded mount pattern: {}", expanded);
        PathBuf::from(expanded)
    }    
}