// SPDX-License-Identifier: MIT
// src/sync/watcher.rs

//! Watches filesystem paths given by DaemonState; no AppConfig or device logic here.

use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::time::{Duration, Instant};

/// Public controller used by DaemonState to add/remove watched sources.
#[derive(Clone, Debug)]
pub struct WatcherControl {
    tx_cmd: Sender<WatcherCommand>,
}

enum WatcherCommand {
    Add(PathBuf),
    Remove(PathBuf),
    ClearAll,
}

/// Start the watcher thread. Returns a handle allowing dynamic updates.
pub fn start_watcher(
    debounce_ms: u64,
    on_batch: impl Fn(Vec<Event>) + Send + 'static,
) -> Result<WatcherControl> {
    let (tx_cmd, rx_cmd) = channel::<WatcherCommand>();
    let (tx_ev, rx_ev) = channel::<Result<Event, notify::Error>>();

    std::thread::spawn(move || {
        let mut watcher =
            RecommendedWatcher::new(tx_ev, notify::Config::default()).expect("watcher");

        let mut watched: Vec<PathBuf> = Vec::new();
        let mut pending: Vec<Event> = Vec::new();
        let mut last: Option<Instant> = None;

        loop {
            // Apply control commands
            if let Ok(cmd) = rx_cmd.try_recv() {
                match cmd {
                    WatcherCommand::Add(path) => {
                        let display_str = path.display().to_string();
                        if !watched.contains(&path) {
                            if watcher.watch(&path, RecursiveMode::Recursive).is_ok() {
                                watched.push(path.clone());
                                tracing::info!("Now watching {}", display_str);
                            }
                        }
                    }
                    WatcherCommand::Remove(path) => {
                        let _ = watcher.unwatch(&path);
                        watched.retain(|p| p != &path);
                        tracing::info!("Stopped watching {}", path.display());
                    }
                    WatcherCommand::ClearAll => {
                        for p in watched.drain(..) {
                            let _ = watcher.unwatch(&p);
                            tracing::info!("Stopped watching {}", p.display());
                        }
                        tracing::info!("Watcher: cleared all paths.");
                    }
                }
            }

            // Read FS events
            if let Ok(Ok(ev)) = rx_ev.try_recv() {
                pending.push(ev);
                last = Some(Instant::now());
            }

            // Debounce flush
            if let Some(t) = last {
                if t.elapsed() >= Duration::from_millis(debounce_ms) {
                    if !pending.is_empty() {
                        let batch = pending.drain(..).collect();
                        on_batch(batch);
                    }
                    last = None;
                }
            }

            std::thread::sleep(Duration::from_millis(10));
        }
    });

    Ok(WatcherControl { tx_cmd })
}

impl WatcherControl {
    pub fn add(&self, path: PathBuf) {
        let _ = self.tx_cmd.send(WatcherCommand::Add(path));
    }

    pub fn remove(&self, path: PathBuf) {
        let _ = self.tx_cmd.send(WatcherCommand::Remove(path));
    }

    pub fn clear_all(&self) {
        let _ = self.tx_cmd.send(WatcherCommand::ClearAll);
    }
}
