use sonik::commands;
use sonik::context::ExecutionContext;

use clap::{CommandFactory, Parser, Subcommand};
use anyhow::Result;
use log::info;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    /// Disable progress bars during sync
    #[arg(long)]
    no_progress_bar: bool,

    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    Run {
        #[arg(short, long)]
        verbose: bool,
    },
    Dump {
        file: String,
        #[arg(short, long)]
        pretty: bool,
    },
    Stats {
        file: String,
    },
    ClearIndex {
        device: String,
    },
    ShowConfig {
        #[arg(short, long)]
        verbose: bool,
    },
    ShowIndexes {
        #[arg(short, long)]
        verbose: bool,
    },
    EditConfig {

    },
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Sonik starting…");

    let cli = Cli::parse();
    let show_progress = !cli.no_progress_bar;

    let ctx = ExecutionContext::from_default_config()?;

    match cli.command {
        Some(Cmd::Run { verbose }) => {
            commands::run::run_now(&ctx, verbose, show_progress)?;
        }

        Some(Cmd::Dump { file, pretty }) => {
            commands::dump::run(&file, pretty)?;
        }

        Some(Cmd::Stats { file }) => {
            commands::stats::run(&file)?;
        }

        Some(Cmd::ClearIndex { device }) => {
            commands::clear::run(&device)?;
        }

        Some(Cmd::ShowConfig { verbose }) => {
            commands::show_config::run(&ctx, verbose)?;
        }
        
        Some(Cmd::ShowIndexes { verbose }) => {
            commands::show_indexes::run(&ctx, verbose)?;
        }

        Some(Cmd::EditConfig { }) => {
            commands::config::run_edit()?;
        }

        None => {
            // print help if no command given
            Cli::command().print_help()?;
            println!();
        }
    }

    Ok(())
}
