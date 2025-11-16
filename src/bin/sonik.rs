// SPDX-License-Identifier: MIT
// src/bin/sonik.rs

//! Main CLI entry point for Sonik application.

use sonik::commands;
use sonik::context::ExecutionContext;

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
    /// Sync commands
    Sync(SyncCommands),

    /// Manage configuration
    Config(ConfigCommands),

    /// Index management
    Index(IndexCommands),
}

#[derive(Parser)]
pub struct SyncCommands {
    #[command(subcommand)]
    pub command: SyncSubcommands,
}

#[derive(Subcommand)]
pub enum SyncSubcommands {
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
    Show ,
    Edit,
}

#[derive(Parser)]
pub struct IndexCommands {
    #[command(subcommand)]
    pub command: IndexSubcommands,
}

#[derive(Subcommand)]
pub enum IndexSubcommands {
    Show {
        device: Option<String>,

        #[arg(short, long)]
        verbose: bool,
    },

    Clear {
        device: String,
    },

    Dump {
        file: String,

        #[arg(short, long)]
        pretty: bool,
    },

    Stats {
        file: String,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Sonik starting…");

    let cli = Cli::parse();
    let ctx = ExecutionContext::from_default_config()?;

    match cli.command {
        Commands::Sync(cmd) => match cmd.command {
            SyncSubcommands::Run { verbose, no_progress_bar } =>
                commands::run::run_now(&ctx, verbose, no_progress_bar)?,
        },

        Commands::Config(cmd) => match cmd.command {
            ConfigSubcommands::Show =>
                commands::config::run_show(&ctx)?,

            ConfigSubcommands::Edit =>
                commands::config::run_edit()?,
        },

        Commands::Index(cmd) => match cmd.command {
            IndexSubcommands::Show { device, verbose } =>
                commands::indexes::run_show(&ctx, device.as_deref(), verbose)?,

            IndexSubcommands::Clear { device } =>
                commands::indexes::run_clear(&ctx, &device)?,

            IndexSubcommands::Dump { file, pretty } =>
                commands::indexes::run_dump(&file, pretty)?,

            IndexSubcommands::Stats { file } =>
                commands::indexes::run_stats(&file)?,
        },
    }

    Ok(())
}
