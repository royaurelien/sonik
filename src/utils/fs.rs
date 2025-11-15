// SPDX-License-Identifier: MIT
// src/utils/fs.rs

//! Filesystem utilities.

use anyhow::Result;
use std::path::{ Path, PathBuf};
use std::fs;

use crate::context::{EnvContext, PathExpander};

/// Ensure path is writable
pub fn ensure_writable(path: &Path) -> anyhow::Result<()> {
    let test = path.join(".sonik_test");
    std::fs::write(&test, b"ok")?;
    std::fs::remove_file(test)?;
    Ok(())
}

/// Copy one file from source to target
pub fn copy_one(source: &Path, target: &Path, file: &PathBuf) -> Result<()> {
    let src = source.join(file);
    let dst = target.join(file);

    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    // std::thread::sleep(std::time::Duration::from_millis(1000)); // Simulate delay
    fs::copy(src, dst)?;
    Ok(())
}

/// Delete one file from device
pub fn delete_one(target: &Path, rel: &PathBuf) -> Result<()> {
    let p = target.join(rel);
    if p.exists() {
        fs::remove_file(p)?;
    }
    Ok(())
}


impl PathExpander {
    pub fn new(ctx: EnvContext) -> Self {
        Self { ctx }
    }

    pub fn expand(&self, input: &str, device: &str) -> PathBuf {
        let mut out = input.to_string();

        // Expand ~/
        if let Some(s) = out.strip_prefix("~/") {
            out = format!("{}/{}", self.ctx.home, s);
        }

        // Replace variables
        let replacements = [
            ("{home}",  self.ctx.home.as_str()),
            ("{user}",  self.ctx.user.as_str()),
            ("{uid}",   self.ctx.uid.as_str()),
            ("{device}", device),
        ];

        for (key, val) in replacements {
            out = out.replace(key, val);
        }

        PathBuf::from(out)
    }
}