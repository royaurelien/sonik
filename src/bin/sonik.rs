// SPDX-License-Identifier: MIT
// src/bin/sonik.rs

//! Main CLI entry point for Sonik application.

use sonik::commands;
use sonik::context::ExecutionContext;

use clap::{CommandFactory, Parser, Subcommand};
use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    /// Disable progress bars during sync
    #[arg(long)]
    no_progress_bar: bool,

    #[arg(short,long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    Run {
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
    },
    ShowIndexes {
        device: String,
    },
    EditConfig {

    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_writer(std::io::stderr)
    .init();
    tracing::info!("Sonik starting…");

    let cli = Cli::parse();
    let show_progress = !cli.no_progress_bar;
    let verbose = cli.verbose;

    let ctx = ExecutionContext::from_default_config()?;

    match cli.command {
        Some(Cmd::Run { }) => {
            commands::run::run_now(&ctx, verbose, show_progress)?;
        }   

        Some(Cmd::Dump { file, pretty }) => {
            commands::dump::run(&file, pretty)?;
        }

        Some(Cmd::Stats { file }) => {
            commands::stats::run(&file)?;
        }

        Some(Cmd::ClearIndex { device }) => {
            commands::indexes::run_clear(&ctx, &device)?;
        }

        Some(Cmd::ShowConfig { }) => {
            commands::config::run_show(&ctx, verbose)?;
        }
        
        Some(Cmd::ShowIndexes { device }) => {
            commands::indexes::run_show(&ctx, &device,verbose)?;
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
