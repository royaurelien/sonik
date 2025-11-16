// SPDX-License-Identifier: MIT
// src/commands/show_indexes.rs

//! Command to show indexes for configured devices.
use std::fs;
use anyhow::Result;

use crate::context::ExecutionContext;
use crate::core::index::Index;
use crate::utils::human::{human_size, human_date};

/// Command to show indexes for configured devices.
pub fn run_show(ctx: &ExecutionContext, device: Option<&str>, verbose: bool) -> Result<()> {
    let all = ctx.config._build_sync_configs(false)?;

    if all.is_empty() {
        println!("No folders configured for sync.");
        return Ok(());
    }

    // Normalize device filter (optional)
    let device_filter = device.unwrap_or("").trim();
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

    println!("Removing all Plainsync index files in {}", target_dir.display());
    fs::remove_dir_all(&target_dir)?;

    println!("Done. Index will rebuild automatically on next sync.");
    Ok(())
}

pub fn run_dump(file: &str, pretty: bool) -> Result<()> {
    let raw = fs::read(file)?;
    let index: Index = bincode::deserialize(&raw)?;

    if !pretty {
        println!("{}", serde_json::to_string(&index)?);
        return Ok(());
    }

    println!("VERSION: {}", index.version);
    println!("GENERATED_AT: {}", human_date(index.generated_at));
    println!("FILES: {}\n", index.files.len());

    println!("{:<20}  {:>12}  {}", "MODIFIED", "SIZE", "PATH");
    println!("{}", "-".repeat(60));

    for f in index.files {
        println!(
            "{:<20}  {:>12}  {}",
            human_date(f.mtime),
            human_size(f.size),
            f.path
        );
    }

    Ok(())
}

/// Command to display statistics about an index file.
pub fn run_stats(file: &str) -> anyhow::Result<()> {
    let raw = fs::read(&file)?;
    let index: Index = bincode::deserialize(&raw)?;

    let total_files = index.files.len();
    let total_size: u64 = index.files.iter().map(|f| f.size).sum();
    let avg_size = if total_files > 0 {
        total_size / total_files as u64
    } else {
        0
    };

    let biggest = index.files.iter().max_by_key(|f| f.size);
    let newest = index.files.iter().max_by_key(|f| f.mtime);
    let oldest = index.files.iter().min_by_key(|f| f.mtime);

    println!("VERSION: {}", index.version);
    println!("GENERATED_AT: {}", human_date(index.generated_at));
    println!("FILES: {}", total_files);
    println!("TOTAL SIZE: {}", human_size(total_size));
    println!("AVERAGE SIZE: {}", human_size(avg_size));

    if let Some(f) = biggest {
        println!("BIGGEST: {} ({})", f.path, human_size(f.size));
    }

    if let Some(f) = newest {
        println!("NEWEST: {} ({})", f.path, human_date(f.mtime));
    }

    if let Some(f) = oldest {
        println!("OLDEST: {} ({})", f.path, human_date(f.mtime));
    }

    println!();
    Ok(())
}