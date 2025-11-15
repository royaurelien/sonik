// SPDX-License-Identifier: MIT
// src/sync/planner.rs

//! Sync planner is responsible for determining which configured sync folders
//! correspond to currently mounted devices.

use anyhow::Result;
use std::collections::HashSet;

use crate::config::SyncConfig;
use crate::context::ExecutionContext;
use crate::sync::detect::detect_all_devices;

/// Compute a list of SyncConfig that point to mounted devices.
/// This is used by both the daemon and the run_now CLI command.
pub fn plan_sync(ctx: &ExecutionContext) -> Result<Vec<SyncConfig>> {
    // Step 1: build all possible sync configs from YAML
    let all = ctx.config.build_sync_configs()?;

    if all.is_empty() {
        return Ok(vec![]);
    }

    // Expand paths from context (e.g., ~ to home directory)
    let all = ctx.expand_paths(all.iter().collect());

    // Step 2: detect currently mounted devices
    let mounted = detect_all_devices(&ctx);

    if mounted.is_empty() {
        return Ok(vec![]);
    }

    let mounted_names: HashSet<&str> =
        mounted.iter().map(|(dev, _)| dev.name.as_str()).collect();

    // Step 3: keep only configs whose device is mounted
    let filtered: Vec<SyncConfig> = all
        .into_iter()
        .filter(|conf| mounted_names.contains(conf.device_name.as_str()))
        .collect();

    Ok(filtered)
}
