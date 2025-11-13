/// bin/sonikd.rs
/// Main binary for Sonik daemon.

use anyhow::Result;
use tracing_subscriber::fmt;

use sonik::config::load_config;
use sonik::daemon::state::DaemonState;
use sonik::sync::detect_loop::{start_detect_loop, DetectCallbacks};
use sonik::sync::engine::SyncEngine;
use sonik::sync::watcher::start_watcher;

fn main() -> Result<()> {
    fmt().with_target(false).with_ansi(false).init();
    tracing::info!("Sonik daemon starting…");

    // Load configuration
    let app_conf = load_config().map_err(|e| {
        tracing::error!("Failed to load config: {e}");
        e
    })?;

    // Create engine (no config inside)
    let engine = SyncEngine::new();

    // Start watcher (dynamic)
    let app_conf_clone = app_conf.clone();

    let watcher = start_watcher(
        app_conf.watch.debounce_ms,
        move |batch| {
            // Batch callback, forward to DaemonState
            let state = DAEMON_STATE.get().unwrap();
            state.handle_events(batch);
        },
    )?;

    // Build daemon state
    static DAEMON_STATE: once_cell::sync::OnceCell<DaemonState> = once_cell::sync::OnceCell::new();
    DAEMON_STATE
        .set(DaemonState::new(app_conf.clone(), engine.clone(), watcher.clone()))
        .unwrap();

    let _state = DAEMON_STATE.get().unwrap();

    // Hot-plug detection
    start_detect_loop(
        app_conf_clone,
        DetectCallbacks {
            on_mount: move |dev| {
                let state = DAEMON_STATE.get().unwrap();
                state.on_device_mounted(&dev);
            },
            on_unmount: move |dev| {
                let state = DAEMON_STATE.get().unwrap();
                state.on_device_unmounted(&dev);
            },
        },
    );

    tracing::info!("Daemon fully started. Watching for devices & changes.");
    std::thread::park();

    Ok(())
}
