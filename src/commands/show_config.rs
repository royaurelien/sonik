/// commands/show_config.rs


use anyhow::Result;
use crate::config::AppConfig;


pub fn run(app_conf: &AppConfig, verbose: bool) -> Result<()> {

    let all = app_conf._build_sync_configs(false)?;

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