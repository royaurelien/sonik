/// core/index.rs
/// Module for handling file indexes.

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

    // #[serde(skip)]
    // exists: bool,

    #[serde(skip)]
    path: PathBuf,
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
            });
        }

        let raw = fs::read(path)?;
        let mut idx: Self = bincode::deserialize(&raw)?;

        idx.path = path.to_path_buf();

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
