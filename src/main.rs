mod config;
mod sync;
mod watcher;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    Run,
    Watch,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let conf = config::load()?;

    match cli.command {
        Some(Cmd::Run) => sync::run(&conf)?,
        Some(Cmd::Watch) => watcher::watch(&conf)?,
        None => sync::run(&conf)?,
    }

    Ok(())
}
