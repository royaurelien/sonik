/// core/scanner.rs
/// Module for scanning local filesystem and producing file indexes.

use crate::core::index::IndexedFile;
use walkdir::WalkDir;
use anyhow::Result;
use std::path::Path;

pub fn scan_local(root: &Path) -> Result<Vec<IndexedFile>> {
    let mut out = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let meta = entry.metadata()?;
            let size = meta.len();
            let mtime = meta.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;

            let rel = entry.path().strip_prefix(root)?.to_string_lossy().to_string();

            out.push(IndexedFile { path: rel, size, mtime });
        }
    }

    Ok(out)
}
