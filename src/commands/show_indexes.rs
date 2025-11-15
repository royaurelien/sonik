/// commands/show_config.rs


use anyhow::Result;
use crate::context::ExecutionContext;
use crate::core::index::Index;

pub fn run(ctx: &ExecutionContext, verbose: bool) -> Result<()> {

    let all = ctx.config._build_sync_configs(false)?;

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