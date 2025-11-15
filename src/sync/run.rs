// SPDX-License-Identifier: MIT
// src/sync/run.rs

//! Perform synchronization for a single folder pair.

use anyhow::Result;
use std::time::Instant;

use crate::config::SyncConfig;
use crate::core::{
    diff::compute_diff,
    index::{Index, IndexedFile},
    scanner::scan_local,
};
use crate::utils::human::{human_size, notify, SyncStats};
use crate::sync::operations::{upload_batch, delete_batch};
use crate::sync::validation::validate_sync_paths;

use indicatif::{ProgressBar, ProgressStyle};

const TEST_WRITE: bool = true;

/// Compute total size of indexed files
fn total_size(files: &[IndexedFile]) -> u64 {
    files.iter().map(|f| f.size).sum()
}

/// Calculate sync statistics from diff and previous index
fn compute_sync_stats(diff: &crate::core::diff::Diff, previous: &[IndexedFile]) -> SyncStats {
    let upload_bytes = total_size(&diff.to_upload);
    let delete_bytes = diff.to_delete.iter()
        .filter_map(|rel| previous.iter().find(|f| f.path == *rel).map(|f| f.size))
        .sum();
    
    SyncStats::new(
        diff.to_upload.len(),
        diff.to_delete.len(),
        upload_bytes,
        delete_bytes
    )
}


/// Perform a full sync for one folder pair
pub fn sync_folder(conf: &SyncConfig, verbose: bool, show_progress: bool) -> Result<()> {
    tracing::debug!("Sync folder called for {}", conf.device_name);
    
    let start = Instant::now();

    // Ensure destination exists and is writable
    validate_sync_paths(&conf.source, &conf.target, TEST_WRITE)?;

    // Load previous index (empty if missing)
    let mut idx = Index::load(&conf.index_path)?; 

    // Scan source
    let local_files = scan_local(&conf.source)?;
    let diff = compute_diff(&local_files, &idx.files);    


    // FIRST RUN: index did not exist, must write it even if diff empty
    if !idx.exists() {
        idx.update(local_files.clone())?;

        tracing::info!("Initialized index for {}.", conf.device_name);

        // If diff empty, nothing more to do
        if diff.to_upload.is_empty() && diff.to_delete.is_empty() {
            return Ok(());
        }
    }

    // Nothing to sync
    if diff.to_upload.is_empty() && diff.to_delete.is_empty() {
        tracing::info!("Nothing to synchronize for {}, everything is up to date.", conf.device_name);
        return Ok(());
    }

    // Compute sync statistics
    let stats = compute_sync_stats(&diff, &idx.files);

    tracing::info!("Preparing sync for {} to {}", conf.source.display(), conf.target.display());
    tracing::info!("Changes: {}", stats.format_summary());

    notify(
        &format!("Sync started for {}", conf.device_name),
        &stats.format_summary()
    );

    // Progress bar
    let total_ops = (diff.to_upload.len() + diff.to_delete.len()) as u64;
    let pb = if show_progress {
        let pb = ProgressBar::new(total_ops);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40} {pos}/{len} | {wide_msg}")
                .unwrap(),
        );
        Some(pb)
    } else {
        None
    };    


    // Perform sync operations using batch functions
    let done_upload = upload_batch(&conf.source, &conf.target, &diff.to_upload, pb.as_ref(), verbose)?;
    let done_delete = delete_batch(&conf.target, &diff.to_delete, pb.as_ref(), verbose)?;

    if let Some(pb) = pb { pb.finish(); }

    // Save updated index
    idx.update(local_files)?;

    let elapsed = start.elapsed();

    tracing::info!(
        "Sync completed in {:.2?}: {} uploaded ({}), {} deleted ({})",
        elapsed,
        done_upload,
        human_size(stats.upload_bytes),
        done_delete,
        human_size(stats.delete_bytes)
    );

    notify(
        &format!("Sync completed for {}", conf.device_name),
        &format!(
            "{} uploaded ({}), {} deleted ({}) in {:.2?}",
            done_upload,
            human_size(stats.upload_bytes),
            done_delete,
            human_size(stats.delete_bytes),
            elapsed
        )
    );

    Ok(())
}
