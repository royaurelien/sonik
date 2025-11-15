// SPDX-License-Identifier: MIT
// src/core/diff.rs

//! Module for computing differences between local files and previous index.

use crate::core::index::IndexedFile;
use std::collections::{HashMap, HashSet};

pub struct Diff {
    pub to_upload: Vec<IndexedFile>,
    pub to_delete: Vec<String>,
}

pub fn compute_diff(local: &[IndexedFile], previous: &[IndexedFile]) -> Diff {
    let mut prev_map = HashMap::new();
    for f in previous {
        prev_map.insert(&f.path, (f.size, f.mtime));
    }

    let mut to_upload = Vec::new();

    // New or updated files
    for lf in local {
        match prev_map.get(&lf.path) {
            Some((size, mtime)) if *size == lf.size && *mtime == lf.mtime => {}
            _ => to_upload.push(lf.clone()),
        }
    }

    // Deleted files
    let local_set: HashSet<_> = local.iter().map(|f| &f.path).collect();
    let mut to_delete = Vec::new();

    for pf in previous {
        if !local_set.contains(&pf.path) {
            to_delete.push(pf.path.clone());
        }
    }

    Diff { to_upload, to_delete }
}
