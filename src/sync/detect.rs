// SPDX-License-Identifier: MIT
// src/sync/detect.rs

//! Utilities for detecting mounted devices based on configuration.

use std::fs;
use std::path::PathBuf;

use crate::config::DeviceConfig;
use crate::context::ExecutionContext;

const PROC_MOUNTINFO: &str = "/proc/self/mountinfo";

/// Parse /proc/self/mountinfo and return all mount points
fn read_mounts() -> Vec<PathBuf> {
    let mut out = Vec::new();

    if let Ok(txt) = fs::read_to_string(PROC_MOUNTINFO) {
        for line in txt.lines() {
            // format: <id> <parent> <major:minor> <root> <mountpoint> <options> ...
            // we only need the 5th field
            if let Some(mp) = line.split_whitespace().nth(4) {
                out.push(PathBuf::from(mp));
            }
        }
    }

    out
}

/// Return list of mounted devices declared in config.yaml
pub fn detect_all_devices(ctx: &ExecutionContext) -> Vec<(DeviceConfig, PathBuf)> {
    let mut out = Vec::new();
    let mounts = read_mounts();

    for dev in &ctx.config.devices {
        // Always produce the expected full mount path
        let expected = ctx.expand_mount(dev);

        match dev.mountinfo {
            true => {
                // USB mount must appear in /proc/self/mountinfo
                for mp in &mounts {
                    if *mp == expected {
                        out.push((dev.clone(), mp.clone()));
                        break;
                    }
                }
            }

            false => {
                // GVFS mount exists ONLY if device is mounted
                if expected.exists() {
                    out.push((dev.clone(), expected.clone()));
                }
            }
        }
    }

    out
}
