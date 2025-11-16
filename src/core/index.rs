// SPDX-License-Identifier: MIT
// src/core/index.rs

//! Handling file indexes, including loading, saving, and updating.

use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::fs;
use std::fmt;
use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::utils::human;

/// Current version of the index file format.
/// Increment when the struct evolves.
const INDEX_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub path: String,
    pub size: u64,
    pub mtime: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub version: u32,
    pub generated_at: i64,
    pub files: Vec<IndexedFile>,

    #[serde(skip)]
    pub path: PathBuf,
    pub total_files: usize,
    pub total_size: u64,
    pub avg_size: u64,
    pub biggest: Option<IndexedFile>,
    pub newest: Option<IndexedFile>,
    pub oldest: Option<IndexedFile>,
}

impl Index {
    /// Load an index and give it awareness of its own location and existence.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                version: INDEX_VERSION,
                generated_at: Utc::now().timestamp(),
                files: vec![],
                path: path.to_path_buf(),
                total_files: 0,
                total_size: 0,
                avg_size: 0,
                biggest: None,
                newest: None,
                oldest: None,                
            });
        }

        let raw = fs::read(path)?;
        let mut idx: Self = bincode::deserialize(&raw)?;

        idx.path = path.to_path_buf();
        idx.total_files = idx.files.len();
        idx.total_size = idx.files.iter().map(|f| f.size).sum();
        idx.avg_size = if idx.total_files > 0 {
            idx.total_size / idx.total_files as u64
        } else {
            0
        };
        idx.biggest = idx.files.iter().max_by_key(|f| f.size).cloned();
        idx.newest = idx.files.iter().max_by_key(|f| f.mtime).cloned();
        idx.oldest = idx.files.iter().min_by_key(|f| f.mtime).cloned();

        Ok(idx)
    }

    /// Save atomically, using the internally stored path.
    pub fn save(&self) -> Result<()> {
        let tmp = self.path.with_extension("tmp");

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let encoded = bincode::serialize(self)?;
        fs::write(&tmp, encoded)?;
        fs::rename(tmp, &self.path)?;

        Ok(())
    }

    /// Update and save index atomically
    pub fn update(&mut self, files: Vec<IndexedFile>) -> Result<()> {
        self.files = files;
        self.generated_at = Utc::now().timestamp();
        self.save()
    }    

    /// Returns whether the index existed before being loaded.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Version: {}, Last index update: {}, files: {}",
            self.version,
            human::human_date(self.generated_at),
            self.files.len(),
        )
    }
}

impl fmt::Display for IndexedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.path,
            human::human_date(self.mtime),
            human::human_size(self.size),
        )
    }
}
