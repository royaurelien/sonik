use std::sync::{Arc, Mutex};
use anyhow::Result;
use notify::Event;

use crate::config::SyncConfig;
use crate::sync::run::sync_folder;

/// Stateless sync engine: it only executes syncs.
/// SyncConfig is now constructed exclusively by AppConfig.
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

    /// Run a synchronization for an existing SyncConfig.
    pub fn sync_config(&self, cfg: &SyncConfig) -> Result<()> {
        let mut lock = self.syncing.lock().unwrap();
        if *lock {
            tracing::warn!(
                "Sync already in progress for device '{}' — skipped.",
                cfg.device_name
            );
            return Ok(());
        }
        *lock = true;
        drop(lock);

        tracing::info!(
            "Starting sync for '{}' to '{}'",
            cfg.source.display(),
            cfg.target.display()
        );

        let result = sync_folder(cfg, false, false);

        let mut lock = self.syncing.lock().unwrap();
        *lock = false;

        result
    }

    /// Match FS events to active SyncConfigs.
    /// Used by DaemonState for watcher batch processing.
    pub fn match_event<'a>(
        &'a self,
        event: &Event,
        active: &'a [SyncConfig],
    ) -> Vec<&'a SyncConfig> {
        let mut out = Vec::new();
        let Some(path) = event.paths.first() else { return out };

        for cfg in active {
            if path.starts_with(&cfg.source) {
                out.push(cfg);
            }
        }

        out
    }
}
