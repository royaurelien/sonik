// SPDX-License-Identifier: MIT
// src/sync/detect_loop.rs

//! Hot-plug detection loop based on /proc/self/mountinfo or equivalent.
//! Calls the provided callbacks whenever a device is mounted or unmounted.

use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use crate::context::ExecutionContext;
use crate::sync::detect::detect_all_devices;

pub struct DetectCallbacks<CbMount, CbUmount>
where
    CbMount: Fn(String) + Send + Sync + 'static,
    CbUmount: Fn(String) + Send + Sync + 'static,
{
    pub on_mount: CbMount,
    pub on_unmount: CbUmount,
}

pub fn start_detect_loop<CbMount, CbUmount>(
    ctx: ExecutionContext,
    callbacks: DetectCallbacks<CbMount, CbUmount>,
) where
    CbMount: Fn(String) + Send + Sync + 'static,
    CbUmount: Fn(String) + Send + Sync + 'static,
{
    thread::spawn(move || {
        let mut previous: HashSet<String> = HashSet::new();

        loop {
            let detected: HashSet<String> = detect_all_devices(&ctx)
                .into_iter()
                .map(|(dev, _)| dev.name)
                .collect();

            let added = detected.difference(&previous);
            let removed = previous.difference(&detected);

            for name in added {
                (callbacks.on_mount)(name.clone());
            }

            for name in removed {
                (callbacks.on_unmount)(name.clone());
            }

            previous = detected;
            thread::sleep(Duration::from_secs(1));
        }
    });
}
