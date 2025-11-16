// SPDX-License-Identifier: MIT
// src/commands/index.rs

//! Commands for inspecting and managing index files.

use std::fs;
use std::path::Path;
use anyhow::Result;
use comfy_table::{Table, presets::{ASCII_FULL}, Cell};

use crate::context::ExecutionContext;
use crate::core::index::Index;
use crate::utils::human::{human_size, human_date, shrink_middle};
use crate::core::task::{SyncTask, SyncTaskFilter, SyncTaskExpand};

const PATH_DISPLAY_LENGTH: usize = 80;

/// Command to list indexes for configured devices.
pub fn run_ls(
    ctx: &ExecutionContext,
    device: Option<&str>,
) -> Result<()> {
    let tasks = ctx.config.load_tasks()?;
    let mut tasks: Vec<SyncTask> = tasks.expanded(ctx);

    // If a device filter is provided, apply it
    if let Some(dev) = device {
        let dev = dev.trim();
        tasks = tasks.clone().by_device(dev);
    }

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(ASCII_FULL)
        .set_header(["Device", "Source", "Target", "Index Path", "Last Modified"]);

    for task in tasks {
        let index = task.load_index()?;

        if !index.exists() {
            // If no index exists, show dashes
            table.add_row([
                &task.device.to_string(),
                task.source.to_str().unwrap_or(""),
                task.target.to_str().unwrap_or(""),
                task.index_path.to_str().unwrap_or(""),
                "-",
            ]);
            continue;
        }

        table.add_row([
            &task.device.to_string(),
            &task.source.display().to_string(),
            &task.target.display().to_string(),
            &task.index_path.display().to_string(),
            &human_date(index.generated_at),
        ]);
    }

    println!("{table}");

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

/// Dump the contents of an index file to the console.
pub fn run_dump(filepath: &str) -> Result<()> {
    // Bypass config, just load index directly
    let idx = Index::load(Path::new(filepath))?;

    if !idx.exists() {
        println!(
            "No index found at {}",
            filepath
        );
        return Ok(());
    }

    let mut t = Table::new();
    t.load_preset(ASCII_FULL)
        .set_header(["Modified", "Size", "Path"]);

    for f in &idx.files {
        t.add_row([
            Cell::new(human_date(f.mtime)),
            Cell::new(human_size(f.size)),
            Cell::new(shrink_middle(&f.path, PATH_DISPLAY_LENGTH)),
        ]);
    }

    println!("{}", filepath);
    println!("{t}");    

    Ok(())
}

/// Command to display statistics about an index file.
pub fn run_stats(ctx: &ExecutionContext, device: Option<&str>) -> anyhow::Result<()> {
    let tasks = ctx.config.load_tasks()?;
    let mut tasks: Vec<SyncTask> = tasks.expanded(ctx);

    // If a device filter is provided, apply it
    if let Some(dev) = device {
        let dev = dev.trim();
        tasks = tasks.clone().by_device(dev);
    }

    if tasks.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    for task in tasks {
        let idx = task.load_index()?;

        if !idx.exists() {
            println!(
                "No index found for device {} at {}",
                task.device,
                task.index_path.display()
            );
            continue;
        }

        // ---- Global stats table ----
        let mut summary = Table::new();
        summary
            .load_preset(ASCII_FULL)
            .set_header(["Metric", "Value"]);

        summary.add_row(["Version", &idx.version.to_string()]);
        summary.add_row(["Last Modified", &human_date(idx.generated_at)]);
        summary.add_row(["Files", &idx.total_files.to_string()]);
        summary.add_row(["Total size", &human_size(idx.total_size)]);
        summary.add_row(["Average size", &human_size(idx.avg_size)]);

        if let Some(f) = &idx.biggest {
            summary.add_row(["Biggest file", &format!("{} ({})", shrink_middle(&f.path, PATH_DISPLAY_LENGTH), human_size(f.size))]);
        }

        if let Some(f) = &idx.newest {
            summary.add_row(["Newest file", &format!("{} ({})", shrink_middle(&f.path, PATH_DISPLAY_LENGTH), human_date(f.mtime))]);
        }

        if let Some(f) = &idx.oldest {
            summary.add_row(["Oldest file", &format!("{} ({})", shrink_middle(&f.path, PATH_DISPLAY_LENGTH), human_date(f.mtime))]);
        }

        println!("\n{}", task.index_path.display());
        println!("{summary}");
    }

    Ok(())
}