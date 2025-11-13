/// commands/run.rs
/// Command to run sync immediately for all configured folders.

use anyhow::Result;
use crate::config::{AppConfig,SyncConfig};
use crate::sync::run::sync_folder;
use crate::sync::planner::plan_sync;

/// Run sync immediately for all devices defined in the config.
pub fn run_now(app_conf: &AppConfig, verbose: bool, show_progress: bool) -> Result<()> {
    println!("Starting Sonik...");

    let plan = plan_sync(app_conf)?;

    if plan.is_empty() {
        println!("No valid sync targets (check mounted devices).");
        return Ok(());
    }

    for conf in plan {
        sync_one(&conf, verbose, show_progress)?;
    }

    println!("Sync complete.");
    Ok(())
}

/// Sync a single mapping (source to target)
fn sync_one(conf: &SyncConfig, verbose: bool, show_progress: bool) -> Result<()> {
    println!(
        "Syncing '{}' to '{}' (device: {})",
        conf.source.display(),
        conf.target.display(),
        conf.device_name,
    );

    match sync_folder(conf, verbose, show_progress) {
        Ok(_) => {
            println!("OK");
            Ok(())
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}
