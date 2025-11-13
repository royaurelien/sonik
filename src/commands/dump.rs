/// commands/dump.rs
/// Command to dump the contents of an index file.

use crate::core::index::Index;
use anyhow::Result;
use std::fs;
use crate::utils::human::{human_size, human_date};

pub fn run(file: &str, pretty: bool) -> Result<()> {
    let raw = fs::read(file)?;
    let index: Index = bincode::deserialize(&raw)?;

    if !pretty {
        println!("{}", serde_json::to_string(&index)?);
        return Ok(());
    }

    println!("VERSION: {}", index.version);
    println!("GENERATED_AT: {}", human_date(index.generated_at));
    println!("FILES: {}\n", index.files.len());

    println!("{:<20}  {:>12}  {}", "MODIFIED", "SIZE", "PATH");
    println!("{}", "-".repeat(60));

    for f in index.files {
        println!(
            "{:<20}  {:>12}  {}",
            human_date(f.mtime),
            human_size(f.size),
            f.path
        );
    }

    Ok(())
}
