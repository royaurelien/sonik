/// commands/show_config.rs


use anyhow::Result;
use crate::config::AppConfig;
use crate::core::index::Index;

pub fn run(app_conf: &AppConfig, verbose: bool) -> Result<()> {

    let all = app_conf._build_sync_configs(false)?;

    if all.is_empty() {
        println!("No folders configured for sync.");
        return Ok(());
    }

    println!("Configured devices for sync:");
    for sync_conf in all {
        let index = Index::load(&sync_conf.index_path)?;
        println!("{}", sync_conf.index_path.display());
        println!("\t{}", index);

        if verbose {
            for entry in &index.files {
                println!("{}", entry);
            }
        }
        
    }
    println!();

    Ok(())
}