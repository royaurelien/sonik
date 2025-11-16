// SPDX-License-Identifier: MIT
// src/utils/paths.rs

//! Path resolution utilities.

use std::path::PathBuf;
use anyhow::{Result, Context};

use crate::context::{EnvContext, PathExpander};

/// Get standard directory with fallback
pub fn data_dir() -> Result<PathBuf> {
    dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("No data directory found"))
        .context("Failed to locate user data directory")
}

/// Get config directory with fallback
pub fn config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("No config directory found"))
        .context("Failed to locate user config directory")
}

/// Get home directory with fallback
pub fn home_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("No home directory found"))
        .context("Failed to locate user home directory")
}

/// Get plainsync data directory
pub fn app_data_dir() -> Result<PathBuf> {
    Ok(data_dir()?.join("plainsync"))
}

/// Get plainsync config directory
pub fn app_config_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("plainsync"))
}

/// Ensure directory exists
pub fn ensure_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

impl PathExpander {
    pub fn new(ctx: EnvContext) -> Self {
        Self { ctx }
    }

    /// Perform all necessary transformations on the paths
    /// so that they can be used in processing.
    pub fn expand(&self, input: &str, device: &str) -> PathBuf {
        let mut out = input.to_string();


        // Expand ~/ prefix
        if let Some(suffix) = out.strip_prefix("~/") {
            out = format!("{}/{}", self.ctx.home, suffix);
        }

        // POSIX-style variables: $HOME, $USER, etc.
        // This allows things like `$HOME/Music`, `$XDG_MUSIC_DIR`, etc.
        out = shellexpand::env_with_context_no_errors(&out, |var| {
            std::env::var(var).ok()
        }).into_owned();

        // Expand custom placeholders: {home}, {user}, {uid}, {device}
        let replacements = [
            ("{home}",  self.ctx.home.as_str()),
            ("{user}",  self.ctx.user.as_str()),
            ("{uid}",   self.ctx.uid.as_str()),
            ("{device}", device),
        ];

        for (key, val) in replacements {
            out = out.replace(key, val);
        }

        // Convert to PathBuf
        let p = PathBuf::from(&out);

        // Absolute path? Keep it as-is.
        if p.is_absolute() {
            return p;
        }

        // Relative path interpret from HOME
        PathBuf::from(&self.ctx.home).join(p)
    }
}
