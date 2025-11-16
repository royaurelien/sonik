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
use crate::context::ExecutionContext;

pub const DEFAULT_CONFIG: &str = include_str!("../assets/default_config.yaml");

/// Top-level configuration
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub devices: Vec<DeviceConfig>,
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

/// Runtime structure used for each sync operation.
/// Holds full DeviceConfig + FolderConfig.
#[derive(Debug, Clone)]
pub struct SyncTask {
    pub device: DeviceConfig,
    pub folder: FolderConfig,

    // runtime-only
    pub index_path: PathBuf,
    pub source: PathBuf,
    pub target: PathBuf,
}

pub trait SyncTaskFilter {
    fn by_device(self, name: &str) -> Vec<SyncTask>;
    fn enabled(self) -> Vec<SyncTask>;
    fn only_devices(self, names: &std::collections::HashSet<&str>) -> Vec<SyncTask>;
}

impl SyncTaskFilter for Vec<SyncTask> {
    fn by_device(self, name: &str) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|c| c.device.name == name)
            .collect()
    }

    fn enabled(self) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|item| item.folder.enabled)
            .collect()
    }

    fn only_devices(self, names: &std::collections::HashSet<&str>) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|item| names.contains(item.device.name.as_str()))
            .collect()
    }

}

impl SyncTask {
    // Return a new SyncTask with expanded source and target paths.
    pub fn expanded(&self, ctx: &ExecutionContext) -> Self {
        SyncTask {
            source: ctx.expander.expand(self.source.to_str().unwrap(), self.device.name.as_str()),
            target: ctx.expander.expand(self.target.to_str().unwrap(), self.device.name.as_str()),
            ..self.clone()
        }
    }
}

pub trait SyncTaskExpand {
    fn expanded(self, ctx: &ExecutionContext) -> Vec<SyncTask>;
}

impl SyncTaskExpand for Vec<SyncTask> {
    fn expanded(self, ctx: &ExecutionContext) -> Vec<SyncTask> {
        self.into_iter()
            .map(|t| t.expanded(ctx))
            .collect()
    }
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

                let source = PathBuf::from(&folder.source);
                let target = PathBuf::from(&device.mount).join(&folder.target);

                out.push(SyncTask {
                    device: device.clone(),
                    folder: folder.clone(),
                    index_path,
                    source,
                    target,
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

