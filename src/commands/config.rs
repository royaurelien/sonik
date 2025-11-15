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
pub fn run_show(ctx: &ExecutionContext, verbose: bool) -> Result<()> {

    let all = ctx.config._build_sync_configs(false)?;

    if all.is_empty() {
        println!("No folders configured for sync.");
        return Ok(());
    }

    println!("Configured devices for sync:");
    for sync_conf in all {
        if verbose {
            println!("- Device: {}", sync_conf.device_name);
            println!("  Source: {}", sync_conf.source.display());
            println!("  Target: {}", sync_conf.target.display());
            println!("  Index:  {}", sync_conf.index_path.display());
        } else {
            println!("{}", sync_conf);
        }
    }
    println!();

    Ok(())
}