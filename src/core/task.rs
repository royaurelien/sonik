// SPDX-License-Identifier: MIT
// src/core/task.rs

//! SyncTask structure representing a sync operation.
//! Holds DeviceConfig and FolderConfig along with runtime paths.

use std::path::PathBuf;

use crate::config::{DeviceConfig, FolderConfig, SyncMode};
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
    pub mode: SyncMode,
}

impl SyncTask {
    // Return a new SyncTask with expanded source and target paths.
    pub fn expanded(&self, ctx: &ExecutionContext) -> Self {
        SyncTask {
            source: ctx.expander.expand(self.source.to_str().unwrap(), &self.device.name),
            target: ctx.expander.expand(self.target.to_str().unwrap(), &self.device.name),
            ..self.clone()
        }
    }

    // Load the index associated with this SyncTask.
    pub fn load_index(&self) -> anyhow::Result<Index> {
        Index::load(&self.index_path)
    }
}

pub trait SyncTaskFilter {
    fn filter_by_device(self, name: &str) -> Vec<SyncTask>;
    fn filter_by_devices(self, names: &std::collections::HashSet<&str>) -> Vec<SyncTask>;
    fn filter_enabled(self) -> Vec<SyncTask>;
}

impl SyncTaskFilter for Vec<SyncTask> {
    // Filter SyncTasks by device name.
    fn filter_by_device(self, name: &str) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|c| c.device.name == name)
            .collect()
    }

    // Filter SyncTasks to only those whose device names are in the provided set.
    fn filter_by_devices(self, names: &std::collections::HashSet<&str>) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|item| names.contains(item.device.name.as_str()))
            .collect()
    }

    // Filter SyncTasks to only those that are enabled.
    fn filter_enabled(self) -> Vec<SyncTask> {
        self.into_iter()
            .filter(|item| item.folder.enabled)
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