// SPDX-License-Identifier: MIT
// src/bin/plainsync.rs

//! Main CLI entry point for Plainsync application.

use plainsync::commands;
use plainsync::context::ExecutionContext;

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run synchronization tasks
    Run {
        #[arg(long)]
        no_progress_bar: bool,

        #[arg(short, long)]
        verbose: bool,
    },

    /// View or edit configuration
    Config(ConfigCommands),

    /// Inspect and manage index files
    Index(IndexCommands),
}

#[derive(Subcommand)]
pub enum SyncSubcommands {
    /// Run synchronization for all configured devices
    Run {
        #[arg(long)]
        no_progress_bar: bool,

        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Parser)]
pub struct ConfigCommands {
    #[command(subcommand)]
    pub command: ConfigSubcommands,
}

#[derive(Subcommand)]
pub enum ConfigSubcommands {
    /// Show the current configuration
    Show ,
    /// Edit the configuration file
    Edit,
}

#[derive(Parser)]
pub struct IndexCommands {
    #[command(subcommand)]
    pub command: IndexSubcommands,
}

#[derive(Subcommand)]
pub enum IndexSubcommands {
    /// List indexed
    Ls {
        device: Option<String>,
    },

    /// Clear all index files for a device
    Clear {
        device: String,
    },

    /// Dump the contents of an index file to the console.
    Dump {
        filepath: String,
    },

    /// Display statistics about an index file.
    Stats {
        device: Option<String>,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Plainsync starting…");

    let cli = Cli::parse();
    let ctx = ExecutionContext::from_default_config()?;

    match cli.command {
        Commands::Run { verbose, no_progress_bar } =>
            commands::run::run_sync(&ctx, verbose, no_progress_bar)?,

        Commands::Config(cmd) => match cmd.command {
            ConfigSubcommands::Show =>
                commands::config::run_show(&ctx)?,

            ConfigSubcommands::Edit =>
                commands::config::run_edit()?,
        },

        Commands::Index(cmd) => match cmd.command {
            IndexSubcommands::Ls { device } =>
                commands::index::run_ls(&ctx, device.as_deref())?,

            IndexSubcommands::Clear { device } =>
                commands::index::run_clear(&ctx, &device)?,

            IndexSubcommands::Dump { filepath } =>
                commands::index::run_dump(&filepath)?,

            IndexSubcommands::Stats { device } =>
                commands::index::run_stats(&ctx, device.as_deref())?,
        },
    }

    Ok(())
}
