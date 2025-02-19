use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qpass")]
#[command(about = "A post-quantum password manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { account: String, password: String },
    Get { account: String, #[arg(short, long)] show: bool },
    List,
    Delete { account: String },
    Generate { length: u8 },
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { account, password } => {
            println!("Adding account: {}", account);
            // Call storage module
        }
        Commands::Get { account, show } => {
            println!("Retrieving account: {}", account);
            // Call crypto and clipboard module
        }
        Commands::List => {
            println!("Listing accounts...");
            // Call storage module
        }
        Commands::Delete { account } => {
            println!("Deleting account: {}", account);
            // Call storage module
        }
        Commands::Generate { length } => {
            println!("Generating password of length: {}", length);
            // Call generator module
        }
    }
}
