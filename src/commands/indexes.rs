// SPDX-License-Identifier: MIT
// src/commands/show_indexes.rs

//! Command to show indexes for configured devices.
use std::fs;
use anyhow::Result;

use crate::context::ExecutionContext;
use crate::core::index::Index;

/// Command to show indexes for configured devices.
pub fn run_show(ctx: &ExecutionContext, device: &str, verbose: bool) -> Result<()> {
    let all = ctx.config._build_sync_configs(false)?;

    if all.is_empty() {
        println!("No folders configured for sync.");
        return Ok(());
    }

    // Normalize device filter (optional)
    let device_filter = device.trim();
    let filtering = !device_filter.is_empty();

    for sync_conf in all {
        // Skip non-matching devices if a filter is provided
        if filtering && sync_conf.device_name != device_filter {
            continue;
        }

        let index = Index::load(&sync_conf.index_path)?;

        if !index.exists() {
            println!("No index file found at {} for device {}", sync_conf.index_path.display(), sync_conf.device_name);
            continue;
        }

        println!("{}", sync_conf.index_path.display());
        println!("{}", index);

        if verbose {
            for entry in &index.files {
                println!("{}", entry);
            }
        }
    }

    println!();
    Ok(())
}

/// Command to clear indexes for a given device.
pub fn run_clear(ctx: &ExecutionContext, device: &str) -> Result<()> {

    let target_dir = ctx.paths.data.join(device);

    if !target_dir.exists() {
        println!("No index folder found for device '{}'", device);
        return Ok(());
    }

    println!("Removing all Sonik index files in {}", target_dir.display());
    fs::remove_dir_all(&target_dir)?;

    println!("Done. Index will rebuild automatically on next sync.");
    Ok(())
}