// SPDX-License-Identifier: MIT
// src/commands/stats.rs

//! Command to display statistics about an index file.

use crate::core::index::Index;
use crate::utils::human::{human_size, human_date};
use std::fs;

/// Command to display statistics about an index file.
pub fn run(file: &str) -> anyhow::Result<()> {
    let raw = fs::read(&file)?;
    let index: Index = bincode::deserialize(&raw)?;

    let total_files = index.files.len();
    let total_size: u64 = index.files.iter().map(|f| f.size).sum();
    let avg_size = if total_files > 0 {
        total_size / total_files as u64
    } else {
        0
    };

    let biggest = index.files.iter().max_by_key(|f| f.size);
    let newest = index.files.iter().max_by_key(|f| f.mtime);
    let oldest = index.files.iter().min_by_key(|f| f.mtime);

    println!("VERSION: {}", index.version);
    println!("GENERATED_AT: {}", human_date(index.generated_at));
    println!("FILES: {}", total_files);
    println!("TOTAL SIZE: {}", human_size(total_size));
    println!("AVERAGE SIZE: {}", human_size(avg_size));

    if let Some(f) = biggest {
        println!("BIGGEST: {} ({})", f.path, human_size(f.size));
    }

    if let Some(f) = newest {
        println!("NEWEST: {} ({})", f.path, human_date(f.mtime));
    }

    if let Some(f) = oldest {
        println!("OLDEST: {} ({})", f.path, human_date(f.mtime));
    }

    println!();
    Ok(())
}