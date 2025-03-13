use clap::{Args, Parser, Subcommand};

use crate::{clipboard::copy_to_clipboard, generator::generate_password};

#[derive(Parser)]
#[command(name = "qpass")]
#[command(about = "A post-quantum password manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        account: String,
        password: String,
    },
    Get {
        account: String,
        #[arg(short, long)]
        show: bool,
    },
    List,
    Delete {
        account: String,
    },
    Generate(GenerateArgs),
}

#[derive(Args, Debug)]
struct GenerateArgs {
    /// The length of the password
    #[arg(default_value = "8")]
    length: usize,
    /// Allowed character types: "a" (lowercase), "A" (uppercase), "n" (numbers), "s" (special)
    #[arg(default_value = "aAn")]
    allow: String,

    /// Copy the generated password to the clipboard
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    copy: bool,
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
        Commands::Generate(args) => match generate_password(args.length, args.allow) {
            Ok(password) => {
                if args.copy {
                   if let Err(e) = copy_to_clipboard(&password) {
                       eprintln!("Failed: {}", e);
                   }
                   println!("The password is copied to the clipboard")
                }else {
                    println!("{}", &password);
                }
            }
            Err(e) => eprint!("Error: {}", e),
        },
    }
}
