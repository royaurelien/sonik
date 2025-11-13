/// sync/operations.rs
/// Reusable sync operations (upload, delete batches)

use anyhow::Result;
use std::path::{Path, PathBuf};
use indicatif::ProgressBar;

use crate::core::index::IndexedFile;
use crate::utils::fs::{copy_one, delete_one};

/// Upload a batch of files with optional progress tracking
pub fn upload_batch(
    source: &Path,
    target: &Path,
    files: &[IndexedFile],
    progress: Option<&ProgressBar>,
    verbose: bool,
) -> Result<usize> {
    let mut count = 0;

    for file in files {
        let rel = PathBuf::from(&file.path);

        if verbose {
            tracing::info!("UPLOAD {}", rel.display());
        }

        copy_one(source, target, &rel)?;
        count += 1;

        if let Some(pb) = progress {
            pb.set_message(format!("upload {}", rel.display()));
            pb.inc(1);
        }
    }

    Ok(count)
}

/// Delete a batch of files with optional progress tracking
pub fn delete_batch(
    target: &Path,
    paths: &[String],
    progress: Option<&ProgressBar>,
    verbose: bool,
) -> Result<usize> {
    let mut count = 0;

    for rel_str in paths {
        let rel = PathBuf::from(rel_str);

        if verbose {
            tracing::info!("DELETE {}", rel.display());
        }

        delete_one(target, &rel)?;
        count += 1;

        if let Some(pb) = progress {
            pb.set_message(format!("delete {}", rel.display()));
            pb.inc(1);
        }
    }

    Ok(count)
}
