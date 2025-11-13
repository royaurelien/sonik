/// utils/fs.rs
/// File system utility functions.

use anyhow::Result;
use std::path::{ Path, PathBuf};
use std::fs;


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
