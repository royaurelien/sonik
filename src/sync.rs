use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;
use fs_extra::file::{copy_with_progress, CopyOptions};
use crate::config::Config;
use std::path::PathBuf;

pub fn run(conf: &Config) -> anyhow::Result<()> {
    let source = PathBuf::from(&conf.source);
    let target = PathBuf::from(&conf.target);

    // total bytes
    let total: u64 = WalkDir::new(&source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum();

    let bar = ProgressBar::new(total);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {bytes}/{total_bytes} {percent}% {eta}")?
    );

    for entry in WalkDir::new(&source) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(&source)?;
            let dest = target.join(rel);

            if let Some(p) = dest.parent() {
                std::fs::create_dir_all(p)?;
            }

            let mut opts = CopyOptions::new();
            opts.overwrite = true;

            copy_with_progress(
                entry.path(),
                &dest,
                &opts,
                |progress| {
                    bar.inc(progress.copied_bytes as u64);
                    fs_extra::file::TransitProcessResult::ContinueOrAbort
                },
            )?;
        }
    }

    bar.finish_with_message("Sync complete");
    Ok(())
}
