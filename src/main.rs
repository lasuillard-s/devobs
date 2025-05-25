use clap::Parser;
use simplelog::*;

/// CLI for obsessed developers.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Log level.
    #[arg(long, default_value_t = LevelFilter::Info)]
    log_level: LevelFilter,
}

async fn _main(args: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    TermLogger::init(
        args.log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    _main(args).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nothing() {
        assert_eq!(1 + 1, 2);
    }
}
