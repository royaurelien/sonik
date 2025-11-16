// SPDX-License-Identifier: MIT
// src/daemon/state.rs

//! State machine, handling device events and sync operations.

use std::sync::{Arc, Mutex};

use crate::config::SyncTask;
use crate::context::ExecutionContext;
use crate::sync::engine::SyncEngine;
use crate::sync::watcher::WatcherControl;
use crate::sync::planner::plan_sync;
use notify::Event;

#[derive(Clone, Debug)]
pub struct DaemonState {
    ctx: Arc<ExecutionContext>,
    engine: SyncEngine,
    watcher: WatcherControl,
    active_syncs: Arc<Mutex<Vec<SyncTask>>>,
}

impl DaemonState {
    pub fn new(ctx: ExecutionContext, engine: SyncEngine, watcher: WatcherControl) -> Self {
        Self {
            ctx: Arc::new(ctx),
            engine,
            watcher,
            active_syncs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Recompute the full sync plan and reconfigure watchers + active list.
    fn refresh_plan(&self, reason: &str) {
        tracing::info!("Daemon: refreshing sync plan ({})...", reason);

        let plan = match plan_sync(&self.ctx) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Plan sync error: {}", e);
                return;
            }
        };

        {
            let mut active = self.active_syncs.lock().unwrap();
            active.clear();
            active.extend(plan.clone());
        }

        // Reconfigure watchers
        self.watcher.clear_all();
        for cfg in &plan {
            self.watcher.add(cfg.source.clone());
        }

        tracing::info!("Daemon: {} active sync(s) after refresh.", plan.len());

        // Initial full-sync on mount or config change
        if !plan.is_empty() && reason == "device mounted" {
            for cfg in plan {
                if let Err(e) = self.engine.sync_config(&cfg) {
                    tracing::error!("Initial sync error for '{}': {}", cfg.device, e);
                }
            }
        }
    }

    /// Called when a device is mounted.
    pub fn on_device_mounted(&self, _device_name: &str) {
        self.refresh_plan("device mounted");
    }

    /// Called when a device is unmounted.
    pub fn on_device_unmounted(&self, _device_name: &str) {
        self.refresh_plan("device unmounted");
    }

    /// Called by watcher after event debounce.
    pub fn handle_events(&self, events: Vec<Event>) {
        let syncs = self.active_syncs();

        let mut impacted = Vec::new();
        for ev in &events {
            let matches = self.engine.match_event(ev, &syncs);
            for cfg in matches {
                impacted.push(cfg.clone());
            }
        }

        impacted.sort_by(|a, b| a.source.cmp(&b.source));
        impacted.dedup_by(|a, b| a.source == b.source);

        if impacted.is_empty() {
            tracing::info!("Batch: no active sync affected.");
            return;
        }

        tracing::info!("Batch: triggering sync for {} folder(s)", impacted.len());

        for cfg in impacted {
            if let Err(e) = self.engine.sync_config(&cfg) {
                tracing::error!("Sync error: {}", e);
            }
        }
    }

    pub fn active_syncs(&self) -> Vec<SyncTask> {
        self.active_syncs.lock().unwrap().clone()
    }
}
