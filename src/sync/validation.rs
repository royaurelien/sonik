/// sync/validation.rs
/// Validation utilities for sync operations

use anyhow::{Result, Context};
use std::path::Path;

use crate::utils::fs::ensure_writable;

/// Validate that a target device is ready for sync
pub fn validate_target(target: &Path, test_write: bool) -> Result<()> {
    if !target.exists() {
        std::fs::create_dir_all(target)
            .with_context(|| format!("Failed to create target directory: {}", target.display()))?;
    }

    if test_write {
        ensure_writable(target)
            .with_context(|| format!("Target is not writable: {}", target.display()))?;
    }

    Ok(())
}

/// Validate that a source path exists and is readable
pub fn validate_source(source: &Path) -> Result<()> {
    if !source.exists() {
        anyhow::bail!("Source path does not exist: {}", source.display());
    }

    if !source.is_dir() {
        anyhow::bail!("Source path is not a directory: {}", source.display());
    }

    Ok(())
}

/// Validate sync configuration paths
pub fn validate_sync_paths(source: &Path, target: &Path, test_write: bool) -> Result<()> {
    validate_source(source)?;
    validate_target(target, test_write)?;
    Ok(())
}
