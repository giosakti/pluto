mod config;

use clap::Parser;
use config::Config;

/// Agnx - A minimal and fast self-hosted runtime for durable and portable AI agents
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,

    /// Port to listen on (overrides config file)
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut config = Config::load(&args.config)?;

    // CLI port overrides config
    if let Some(port) = args.port {
        config.port = port;
    }

    println!("Starting server on {}:{}", config.host, config.port);
    Ok(())
}
