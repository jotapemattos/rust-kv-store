use clap::{Parser, Subcommand};
use kvs::{KvStore, Result};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set { key: String, value: String },
    /// Get the string value of a given string key
    Get { key: String },
    /// Remove a given key
    Rm { key: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { key, value } => {
            let mut store = KvStore::open(std::env::current_dir()?)?;
            store.set(key, value)?;
        }
        Commands::Get { .. } => todo!("implemented in Part 4"),
        Commands::Rm { .. } => todo!("implemented in step 7"),
    }

    Ok(())
}
