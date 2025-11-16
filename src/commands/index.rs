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

const PATH_DISPLAY_LENGTH: usize = 80;

/// Command to list indexes for configured devices.
pub fn run_ls(
    ctx: &ExecutionContext,
    device: Option<&str>,
) -> Result<()> {
    let all = ctx.config._build_sync_configs(false)?;
    let all = ctx.expand_paths(all.iter().collect());

    if all.is_empty() {
        println!("No folders configured for sync.");
        return Ok(());
    }

    let device_filter = device.unwrap_or("").trim();
    let filtering = !device_filter.is_empty();

    let mut table = Table::new();
    table
        .load_preset(ASCII_FULL)
        .set_header(["Device", "Source", "Target", "Index Path", "Last Modified", "Files", "Total Size"]);

    for sync_conf in all {
        if filtering && sync_conf.device_name != device_filter {
            continue;
        }

        let index = Index::load(&sync_conf.index_path)?;

        if !index.exists() {
            // If no index exists, show dashes
            table.add_row([
                sync_conf.device_name.as_str(),
                sync_conf.source.display().to_string().as_str(),
                sync_conf.target.display().to_string().as_str(),
                sync_conf.index_path.display().to_string().as_str(),
                "-",
                "-",
                "-",
            ]);
            continue;
        }

        table.add_row([
            sync_conf.device_name.as_str(),
            &sync_conf.source.display().to_string(),
            &sync_conf.target.display().to_string(),
            &sync_conf.index_path.display().to_string(),
            &human_date(index.generated_at),
            &index.total_files.to_string(),
            &crate::utils::human::human_size(index.total_size),
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
    let all = ctx.config._build_sync_configs(false)?;

    if all.is_empty() {
        println!("No folders configured for sync.");
        return Ok(());
    }

    let device_filter = device.unwrap_or("").trim();
    let filtering = !device_filter.is_empty();

    for sync_conf in all {
        if filtering && sync_conf.device_name != device_filter {
            continue;
        }

        let idx = Index::load(&sync_conf.index_path)?;

        if !idx.exists() {
            println!(
                "No index found for device {} at {}",
                sync_conf.device_name,
                sync_conf.index_path.display()
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

        println!("{}", sync_conf.index_path.display());
        println!("{summary}");
    }

    Ok(())
}