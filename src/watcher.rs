use notify::{recommended_watcher, RecursiveMode, Watcher, Event};
use std::path::PathBuf;
use crate::{sync, config::Config};

pub fn watch(conf: &Config) -> anyhow::Result<()> {
    let mount_dir = PathBuf::from("/media").join(whoami::username());

    println!("Watching {:?}", mount_dir);

    let mut watcher = recommended_watcher(move |res: Result<Event, _>| {
        if let Ok(event) = res {
            if event.paths.iter().any(|p| p.starts_with(&conf.target)) {
                let _ = sync::run(conf);
            }
        }
    })?;

    watcher.watch(&mount_dir, RecursiveMode::Recursive)?;
    std::thread::park();
    Ok(())
}
