// SPDX-License-Identifier: MIT
// src/sync/planner.rs

//! Sync planner is responsible for determining which configured sync folders
//! correspond to currently mounted devices.

use anyhow::Result;
use std::collections::HashSet;

use crate::context::ExecutionContext;
use crate::sync::detect::detect_all_devices;
use crate::core::task::{SyncTask, SyncTaskFilter, SyncTaskExpand};

/// Compute a list of SyncTask that point to mounted devices.
/// This is used by both the daemon and the run_sync CLI command.
pub fn plan_sync(ctx: &ExecutionContext) -> Result<Vec<SyncTask>> {
    // Step 1: build all possible sync configs from YAML
    let tasks = ctx.config.load_tasks()?.filter_enabled();

    // Expand paths from context (e.g., ~ to home directory)
    let tasks = tasks.expanded(ctx);

    if tasks.is_empty() {
        return Ok(vec![]);
    }


    // Step 2: detect currently mounted devices
    let mounted = detect_all_devices(&ctx);

    if mounted.is_empty() {
        return Ok(vec![]);
    }

    // Step 3: keep only tasks whose device is mounted
    let mounted_names: HashSet<&str> =
        mounted.iter().map(|(dev, _)| dev.name.as_str()).collect();

    let filtered = tasks.filter_by_devices(&mounted_names);

    Ok(filtered)
}
