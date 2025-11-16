// SPDX-License-Identifier: MIT
// src/commands/config.rs

//! Commands related to configuration.

use anyhow::Result;
use crate::context::ExecutionContext;
use crate::utils::editor::open_in_default_editor;
use crate::config::AppConfig;

pub fn run_edit() -> anyhow::Result<()> {
    let conf_path = AppConfig::filepath()?;

    open_in_default_editor(&conf_path)?;

    Ok(())
}

// Show the current sync configuration.
pub fn run_show(ctx: &ExecutionContext) -> Result<()> {
    let sync_confs = ctx.config._build_sync_configs(false)?;
    let devices = &ctx.config.devices;

    if devices.is_empty() {
        println!("No devices configured.");
        return Ok(());
    }

    for dev in devices {
        
        println!("\nDevices:");
        println!("  Name: {}", dev);
        println!("  Mount path: {}", ctx.expand_mount(&dev).to_str().unwrap_or(""));
        println!("  Use mountinfo: {}", dev.mountinfo);


        // Now find matching SyncConfigs (expanded folders, absolute paths, index paths)
        let matching: Vec<_> = sync_confs
            .iter()
            .filter(|sc| sc.device_name == dev.name)
            .collect();

        if matching.is_empty() {
            println!("  (No resolved sync folders for this device)");
            continue;
        }

        let matching = ctx.expand_paths(matching);

        println!("  Folders:");

        for sc in matching {
            println!("    Folder:");
            println!("      Source: {}", sc.source.display());
            println!("      Target: {}", sc.target.display());
            println!("      Index : {}", sc.index_path.display());
            println!("      Enabled : {}", sc.enabled);
        }
    }

    println!();
    Ok(())
}

