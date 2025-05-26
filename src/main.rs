use clap::{Parser, Subcommand};
use simplelog::*;

mod check_file_pair;
mod preferred_suffix;

/// CLI for obsessed developers.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Log level.
    #[arg(global = true, long, default_value_t = LevelFilter::Info)]
    log_level: LevelFilter,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CheckFilePair(check_file_pair::CommandArgs),
    PreferredSuffix(preferred_suffix::CommandArgs),
}

async fn _main(args: Cli) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    TermLogger::init(
        args.log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    // Check the command and execute it
    match args.command {
        Commands::CheckFilePair(args) => check_file_pair::command(args),
        Commands::PreferredSuffix(args) => preferred_suffix::command(args),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();
    _main(args).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nothing() {
        assert_eq!(1 + 1, 2);
    }
}
