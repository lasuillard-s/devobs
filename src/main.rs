mod commands;
#[cfg(test)]
#[path = "../tests/test_helpers.rs"]
pub(crate) mod test_helpers;
mod utils;

use std::{cmp::max, env::current_dir};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

/// CLI for obsessed developers.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, Args, Copy)]
struct GlobalOpts {
    /// Enable debug mode. This will increase the verbosity and detail of the logs.
    #[arg(global = true, long, default_value_t = false)]
    debug: bool,

    /// Set the log level for the application.
    ///
    /// If `debug` is enabled, the minimum log level will be set to `Debug`.
    #[arg(global = true, long, default_value_t = LevelFilter::Info)]
    log_level: LevelFilter,

    /// Disable colored output in the logs.
    #[arg(global = true, long, default_value_t = false)]
    no_colors: bool,

    /// Dry run mode. If enabled, the application behavior will be changed to
    /// not perform any destructive actions.
    #[arg(global = true, long, default_value_t = false)]
    dry_run: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    CheckFilePair(crate::commands::check_file_pair::CommandArgs),
    PreferredSuffix(crate::commands::preferred_suffix::CommandArgs),
}

// TODO(lasuillard): Customize log formatter
async fn _main(args: Cli) -> Result<()> {
    let global_opts = args.global_opts;

    // If debug mode is enabled, set the log level minimum to Debug
    let log_level = if global_opts.debug {
        max(LevelFilter::Debug, global_opts.log_level)
    } else {
        global_opts.log_level
    };

    // Build logging config based on debug / log-level options
    let mut config_builder = simplelog::ConfigBuilder::new();
    if global_opts.debug {
        config_builder.set_time_level(LevelFilter::Error);
        config_builder.set_time_format_rfc3339();
        config_builder.set_thread_level(LevelFilter::Error);
        config_builder.set_target_level(LevelFilter::Error);
        config_builder.set_location_level(LevelFilter::Error);
    } else {
        config_builder.set_time_level(LevelFilter::Off);
        config_builder.set_thread_level(LevelFilter::Off);
        config_builder.set_target_level(LevelFilter::Off);
        config_builder.set_location_level(LevelFilter::Off);
    }

    // Color output
    let color_choice = if global_opts.no_colors {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    };

    // Initialize the logger
    TermLogger::init(
        log_level,
        config_builder.build(),
        TerminalMode::Mixed,
        color_choice,
    )?;

    // Check the command and execute it
    log::trace!("Parsed arguments: {args:?}");
    log::trace!("Global options: {global_opts:?}");
    log::trace!("Running command {:?} at {:?}", args.command, current_dir());
    match args.command {
        Commands::CheckFilePair(args) => {
            crate::commands::check_file_pair::command(args, global_opts)
        }
        Commands::PreferredSuffix(args) => {
            crate::commands::preferred_suffix::command(args, global_opts)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    _main(args).await
}
