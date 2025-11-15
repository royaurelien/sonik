// SPDX-License-Identifier: MIT
// src/commands/clear.rs

//! Command to clear indexes for a given device.

use std::fs;

pub fn run(device: &str) -> anyhow::Result<()> {
    let base = dirs::data_dir().unwrap().join("sonik");
    let target_dir = base.join(&device);

    if !target_dir.exists() {
        println!("No index folder found for device '{}'", device);
        return Ok(());
    }

    println!("Clearing indexes under {}", target_dir.display());

    for entry in walkdir::WalkDir::new(&target_dir)
        .min_depth(1)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = entry.path();

        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("json") {
            println!("  deleting {}", p.display());
            let _ = fs::remove_file(p);
        }
    }

    println!("Done.");
    Ok(())
}