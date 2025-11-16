// SPDX-License-Identifier: MIT
// src/sync/mod.rs

//! Sync module for Plainsync application.

pub mod detect;
pub mod detect_loop;
pub mod engine;
pub mod watcher;
pub mod run;
pub mod operations;
pub mod validation;
pub mod planner;

pub use detect::detect_all_devices;
pub use detect_loop::start_detect_loop;
pub use engine::SyncEngine;
pub use run::sync_folder;