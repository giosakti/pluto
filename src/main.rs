use clap::Parser;

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

fn main() {
    let args = Args::parse();
    println!("Config: {}", args.config);
    if let Some(port) = args.port {
        println!("Port: {}", port);
    }
}
