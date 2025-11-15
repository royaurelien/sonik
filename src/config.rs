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


impl AppConfig {
    /// Return path: ~/.config/sonik/config.yaml
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
    pub fn build_sync_configs(&self) -> Result<Vec<SyncConfig>> {
        self._build_sync_configs(true)
    }

    /// Same logic, but allows including inactive folders (for diagnostics)
    pub fn _build_sync_configs(&self, active_only: bool) -> Result<Vec<SyncConfig>> {
        let mut out = Vec::new();

        let data_base = paths::app_data_dir()?;

        for device in &self.device {
            for folder in &device.folders {
                if active_only && !folder.enabled {
                    continue;
                }

                let index_path = data_base
                    .join(&device.name)
                    .join(format!("{}.bin", unique_slug(&folder.target, &device.name)));

                let source_path = PathBuf::from(&folder.source);
                let target_path = PathBuf::from(&device.mount).join(&folder.target);

                out.push(SyncConfig {
                    device_name: device.name.clone(),
                    source: source_path,
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


