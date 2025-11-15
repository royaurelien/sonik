// SPDX-License-Identifier: MIT
// src/bin/sonikd.rs

//! Main daemon entry point for Sonik application.

use anyhow::Result;
use tracing_subscriber::fmt;

use sonik::daemon::state::DaemonState;
use sonik::sync::detect_loop::{start_detect_loop, DetectCallbacks};
use sonik::sync::engine::SyncEngine;
use sonik::sync::watcher::start_watcher;
use sonik::context::ExecutionContext;

// À mettre au niveau module, pas dans main()
static DAEMON_STATE: once_cell::sync::OnceCell<DaemonState> = once_cell::sync::OnceCell::new();

fn main() -> Result<()> {
    fmt().with_target(false).with_ansi(false).init();
    tracing::info!("Sonik daemon starting…");

    // 1. Build execution context
    let ctx = ExecutionContext::from_default_config()?;
    let ctx_for_detect = ctx.clone(); // clone for detect_loop

    // 2. Create engine
    let engine = SyncEngine::new();

    // 3. Start watcher (only need debounce)
    let watcher = start_watcher(
        ctx.config.watch.debounce_ms,
        move |batch| {
            // Batch callback, forward to DaemonState
            let state = DAEMON_STATE.get().unwrap();
            state.handle_events(batch);
        },
    )?;

    // 4. Build daemon state (ctx is MOVED here)
    DAEMON_STATE
        .set(DaemonState::new(ctx, engine, watcher))
        .unwrap();

    // 5. Hot-plug detection (uses the CLONE of ctx)
    start_detect_loop(
        ctx_for_detect,
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

    // Notify systemd that we're ready
    sd_notify::notify(false, &[sd_notify::NotifyState::Ready]).ok();

    std::thread::park();

    Ok(())
}
