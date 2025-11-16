// SPDX-License-Identifier: MIT
// src/commands/run.rs

//! Command to run sync immediately for all configured folders.

use anyhow::Result;
use crate::config::{SyncConfig};
use crate::context::ExecutionContext;
use crate::sync::run::sync_folder;
use crate::sync::planner::plan_sync;

/// Run sync immediately for all devices defined in the config.
pub fn run_now(ctx: &ExecutionContext, verbose: bool, no_progress: bool) -> Result<()> {
    tracing::info!("Starting Plainsync run");

    let plan = plan_sync(ctx)?;

    if plan.is_empty() {
        println!("No valid sync targets (check mounted devices).");
        return Ok(());
    }

    for conf in plan {
        sync_one(&conf, verbose, !no_progress)?;
    }

    Ok(())
}

/// Sync a single mapping (source to target)
fn sync_one(conf: &SyncConfig, verbose: bool, show_progress: bool) -> Result<()> {
    println!(
        "Checking local folder '{}'.",
        conf.source.display(),
    );

    match sync_folder(conf, verbose, show_progress) {
        Ok(_) => {
            println!("Synchronization completed.");
            Ok(())
        }
        Err(e) => {
            println!("Error syncing device '{}': {}", conf.device_name, e);
            Err(e)
        }
    }
}
