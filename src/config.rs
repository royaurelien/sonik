// SPDX-License-Identifier: MIT
// src/config.rs

//! Manage application configuration.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::utils::slug::unique_slug;
use crate::utils::paths;
use crate::core::task::SyncTask;

pub const DEFAULT_CONFIG: &str = include_str!("../assets/default_config.yaml");

/// Watcher configuration
#[derive(Debug, Deserialize, Clone)]
pub struct WatchConfig {
    pub enabled: bool,
    pub debounce_ms: u64,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum SyncMode {
    Push,
    Pull,
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
    pub mode: Option<SyncMode>, // None = push (backward compatible)
}

/// Top-level configuration
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub devices: Vec<DeviceConfig>,
    pub watch: WatchConfig,
}

impl AppConfig {
    /// Return path: ~/.config/plainsync/config.yaml
    pub fn filepath() -> Result<PathBuf> {
        let base = paths::app_config_dir()?;
        Ok(base.join("config.yaml"))
    }

    /// Create config file + parent directories
    fn bootstrap(path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            paths::ensure_dir(&parent.to_path_buf())?;
        }

        fs::write(path, DEFAULT_CONFIG)
            .with_context(|| format!("Failed to write default config into {}", path.display()))?;

        Ok(())
    }

    /// Load configuration, creating file if missing
    pub fn load() -> Result<Self> {
        let path = Self::filepath()?;

        if !path.exists() {
            Self::bootstrap(&path)?;
        }

        let txt = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config: {}", path.display()))?;

        let conf: Self = serde_yaml::from_str(&txt)
            .with_context(|| format!("Invalid YAML in {}", path.display()))?;

        Ok(conf)
    }

    /// Build list of active sync configurations
    pub fn load_tasks(&self) -> Result<Vec<SyncTask>> {

        let mut out = Vec::new();
        let data_base = paths::app_data_dir()?;

        for device in &self.devices {
            for folder in &device.folders {

                let index_path = data_base
                    .join(&device.name)
                    .join(format!("{}.bin", unique_slug(&folder.target, &device.name)));
                
                // Push or Pull, depending on folder.mode (default to Push)
                let mode = folder.mode.unwrap_or(SyncMode::Push);

                let (source, target) = match mode {
                    SyncMode::Push => {
                        // source = host, target = device
                        let src = PathBuf::from(&folder.source);
                        let dst = PathBuf::from(&device.mount).join(&folder.target);
                        (src, dst)
                    }
                    SyncMode::Pull => {
                        // source = device, target = host
                        let src = PathBuf::from(&device.mount).join(&folder.source);
                        let dst = PathBuf::from(&folder.target);
                        (src, dst)
                    }
                };

                out.push(SyncTask {
                    device: device.clone(),
                    folder: folder.clone(),
                    index_path,
                    source,
                    target,
                    mode
                });
            }
        }

        Ok(out)
    }
}


impl fmt::Display for SyncTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} | {}] {} → {} (index: {})",
            self.device.name,
            if self.folder.enabled { "enabled" } else { "disabled" },
            self.source.display(),
            self.target.display(),
            self.index_path.display(),
        )
    }
}

/// Display device with optional label
impl fmt::Display for DeviceConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(label) = &self.label {
            write!(f, "{} ({})", label, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

