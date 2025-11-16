// SPDX-License-Identifier: MIT
// src/core/task.rs

//! SyncTask structure representing a sync operation.
//! Holds DeviceConfig and FolderConfig along with runtime paths.

use std::path::PathBuf;

use crate::config::{DeviceConfig, FolderConfig};
use crate::context::ExecutionContext;
use crate::core::index::Index;

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

impl SyncTask {
    // Return a new SyncTask with expanded source and target paths.
    pub fn expanded(&self, ctx: &ExecutionContext) -> Self {
        let src = self.source.to_string_lossy();
        let tgt = self.target.to_string_lossy();        
        SyncTask {
            source: ctx.expander.expand(&src, self.device.name.as_str()),
            target: ctx.expander.expand(&tgt, self.device.name.as_str()),
            ..self.clone()
        }
    }

    // Load the index associated with this SyncTask.
    pub fn load_index(&self) -> anyhow::Result<Index> {
        Index::load(&self.index_path)
    }
}

pub trait SyncTaskFilter {
    fn by_device(self, name: &str) -> Vec<SyncTask>;
    fn enabled(self) -> Vec<SyncTask>;
    fn only_devices(self, names: &std::collections::HashSet<&str>) -> Vec<SyncTask>;
}

impl SyncTaskFilter for Vec<SyncTask> {
    // Filter SyncTasks by device name.
    fn by_device(self, name: &str) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|c| c.device.name == name)
            .collect()
    }

    // Filter SyncTasks to only those that are enabled.
    fn enabled(self) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|item| item.folder.enabled)
            .collect()
    }

    // Filter SyncTasks to only those whose device names are in the provided set.
    fn only_devices(self, names: &std::collections::HashSet<&str>) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|item| names.contains(item.device.name.as_str()))
            .collect()
    }

}

pub trait SyncTaskExpand {
    fn expanded(self, ctx: &ExecutionContext) -> Vec<SyncTask>;
}

impl SyncTaskExpand for Vec<SyncTask> {
    // Expand paths for a list of SyncTasks.
    fn expanded(self, ctx: &ExecutionContext) -> Vec<SyncTask> {
        self.into_iter()
            .map(|t| t.expanded(ctx))
            .collect()
    }
}