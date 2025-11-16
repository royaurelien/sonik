// SPDX-License-Identifier: MIT
// src/commands/config.rs

//! Commands related to configuration.

use anyhow::Result;
use crate::context::ExecutionContext;
use crate::utils::editor::open_in_default_editor;
use crate::config::AppConfig;
use crate::core::task::{SyncTask, SyncTaskFilter, SyncTaskExpand};

pub fn run_edit() -> anyhow::Result<()> {
    let conf_path = AppConfig::filepath()?;

    open_in_default_editor(&conf_path)?;

    Ok(())
}

// Show the current sync configuration.
pub fn run_show(ctx: &ExecutionContext) -> Result<()> {
    let tasks = ctx.config.load_tasks()?;
    let tasks: Vec<SyncTask> = tasks.expanded(ctx);
    let devices = &ctx.config.devices;

    if devices.is_empty() {
        println!("No devices configured.");
        return Ok(());
    }

    for dev in devices {
        
        println!("\nDevice:");
        println!("  Name: {}", dev);
        println!("  Mount path: {}", ctx.expand_mount(&dev).to_str().unwrap_or(""));
        println!("  Use mountinfo: {}", dev.mountinfo);


        // Now find matching SyncTasks for this device
        let filtered: Vec<_> = tasks.clone().by_device(&dev.name);

        if filtered.is_empty() {
            println!("  (No resolved sync folders for this device)");
            continue;
        }

        println!("  Folders:");

        for task in filtered {
            println!("    Folder:");
            println!("      Source: {}", task.source.display());
            println!("      Target: {}", task.target.display());
            println!("      Index : {}", task.index_path.display());
            println!("      Enabled : {}", task.folder.enabled);
        }
    }

    println!();
    Ok(())
}

