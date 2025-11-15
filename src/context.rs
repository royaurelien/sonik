// SPDX-License-Identifier: MIT
// src/context.rs

//! Execution context for Sonik application, including configuration, 
//! environment details and path expansion.

use users::{get_current_uid, get_user_by_uid};
use std::env;
use std::path::PathBuf;

use crate::config::{AppConfig, SyncConfig, DeviceConfig};

#[derive(Clone, Debug)]
pub struct EnvContext {
    pub home: String,
    pub user: String,
    pub uid: String,
}

#[derive(Debug, Clone)]
pub struct PathExpander {
    pub ctx: EnvContext,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub config: AppConfig,
    pub env: EnvContext,
    pub expander: PathExpander,
}

impl EnvContext {
    pub fn gather() -> Self {
        let home = env::var("HOME").unwrap_or_else(|_| "/".into());

        let uid_raw = get_current_uid();
        let uid = uid_raw.to_string();

        let user = get_user_by_uid(uid_raw)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into());

        Self { home, user, uid }
    }
}

impl ExecutionContext {
    pub fn new(config: AppConfig) -> Self {
        let env = EnvContext::gather();
        let expander = PathExpander::new(env.clone());
        Self { config, env, expander }
    }

    pub fn from_default_config() -> Result<Self, anyhow::Error> {
        let config = match AppConfig::load() {
            Ok(conf) => conf,
            Err(e) => {
                tracing::error!("Failed to load config, using built-in default: {}", e);
                panic!("DEFAULT_CONFIG is invalid YAML: {}", e);
            }
        };

        Ok(Self::new(config))
    
    }

    pub fn expand_path(&self, raw: &SyncConfig) -> SyncConfig {
        SyncConfig {
            device_name: raw.device_name.clone(),
            source: self.expander.expand(raw.source.to_str().unwrap(), raw.device_name.as_str()),
            target: self.expander.expand(raw.target.to_str().unwrap(), raw.device_name.as_str()),
            index_path: raw.index_path.clone(),
            enabled: raw.enabled,
        }
    }
    pub fn expand_paths(&self, raw: Vec<&SyncConfig>) -> Vec<SyncConfig> {
        raw.into_iter().map(|conf| self.expand_path(conf)).collect()
    }

    pub fn expand_mount(&self, dev: &DeviceConfig) -> PathBuf {
        self.expander.expand(&dev.mount, dev.name.as_str())
    }    
     
}
