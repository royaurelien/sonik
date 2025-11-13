// src/sync/planner.rs

use anyhow::Result;
use std::collections::HashSet;

use crate::config::{AppConfig, SyncConfig};
use crate::sync::detect::detect_all_devices;

/// Compute a list of SyncConfig that point to mounted devices.
/// This is used by both the daemon and the run_now CLI command.
pub fn plan_sync(app_conf: &AppConfig) -> Result<Vec<SyncConfig>> {
    // Step 1: build all possible sync configs from YAML
    let all = app_conf.build_sync_configs()?;

    if all.is_empty() {
        return Ok(vec![]);
    }

    // Step 2: detect currently mounted devices
    let mounted = detect_all_devices(app_conf);

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
