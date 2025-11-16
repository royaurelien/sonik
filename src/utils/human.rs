// SPDX-License-Identifier: MIT
// src/utils/human.rs

//! Human-readable formatting utilities.

use chrono::{TimeZone, Local};

/// Format bytes human-readably (single unified implementation)
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let mut size = bytes as f64;
    let mut unit = 0usize;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

pub fn human_date(ts: i64) -> String {
    let dt = Local.timestamp_opt(ts, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn notify(summary: &str, body: &str) {
    notify_rust::Notification::new()
        .appname("Plainsync")
        .summary(summary)
        .body(body)
        .show()
        .ok();
}

/// Statistics for a sync operation
#[derive(Debug, Clone, Default)]
pub struct SyncStats {
    pub upload_count: usize,
    pub delete_count: usize,
    pub upload_bytes: u64,
    pub delete_bytes: u64,
}

impl SyncStats {
    pub fn new(upload_count: usize, delete_count: usize, upload_bytes: u64, delete_bytes: u64) -> Self {
        Self { upload_count, delete_count, upload_bytes, delete_bytes }
    }

    pub fn has_changes(&self) -> bool {
        self.upload_count > 0 || self.delete_count > 0
    }

    pub fn format_summary(&self) -> String {
        format!(
            "{} upload(s), {} delete(s) (+{}, -{})",
            self.upload_count,
            self.delete_count,
            human_size(self.upload_bytes),
            human_size(self.delete_bytes)
        )
    }
}