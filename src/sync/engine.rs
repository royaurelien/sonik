// SPDX-License-Identifier: MIT
// src/sync/engine.rs

//! Sync engine responsible for executing synchronization operations.

use std::sync::{Arc, Mutex};
use anyhow::Result;
use notify::Event;

use crate::core::task::SyncTask;
use crate::sync::run::sync_folder;

/// Stateless sync engine: it only executes syncs.
#[derive(Debug, Clone)]
pub struct SyncEngine {
    syncing: Arc<Mutex<bool>>,
}

impl SyncEngine {
    pub fn new() -> Self {
        Self {
            syncing: Arc::new(Mutex::new(false)),
        }
    }

    /// Run a synchronization for an existing SyncTask.
    pub fn sync_config(&self, task: &SyncTask) -> Result<()> {
        let mut lock = self.syncing.lock().unwrap();
        if *lock {
            tracing::warn!(
                "Sync already in progress for device '{}' — skipped.",
                task.device
            );
            return Ok(());
        }
        *lock = true;
        drop(lock);

        tracing::info!(
            "Starting sync for '{}' to '{}'",
            task.source.display(),
            task.target.display()
        );

        let result = sync_folder(task, false, false);

        let mut lock = self.syncing.lock().unwrap();
        *lock = false;

        result
    }

    /// Match FS events to active SyncTasks.
    /// Used by DaemonState for watcher batch processing.
    pub fn match_event<'a>(
        &'a self,
        event: &Event,
        active: &'a [SyncTask],
    ) -> Vec<&'a SyncTask> {
        let mut out = Vec::new();
        let Some(path) = event.paths.first() else { return out };

        for task in active {
            if path.starts_with(&task.source) {
                out.push(task);
            }
        }

        out
    }
}
