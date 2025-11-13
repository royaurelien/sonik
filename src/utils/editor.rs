use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Open the config file in the system's default editor.
/// - Creates the file if missing.
/// - Uses xdg-open (Linux), open (macOS), start (Windows).
pub fn open_in_default_editor(path: &Path) -> Result<()> {
    // Ensure parent directories exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }

    // Ensure file exists
    if !path.exists() {
        fs::write(path, "")
            .with_context(|| format!("Failed to create file: {}", path.display()))?;
    }

    #[cfg(target_os = "linux")]
    let mut cmd = {
        let mut c = Command::new("xdg-open");
        c.arg(path);
        c
    };

    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = Command::new("open");
        c.arg(path);
        c
    };

    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.args(["/C", "start", ""]);
        c.arg(path);
        c
    };

    cmd.spawn()
        .with_context(|| format!("Failed to launch editor for {}", path.display()))?;

    Ok(())
}
