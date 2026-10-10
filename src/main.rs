use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "clitoolthing",
    version,
    about = "Trial to see how CLI tools work"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Greet {
        name: String,

        #[arg(short, long, default_value_t = 1)]
        times: u8,
    },

    Count {
        path: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name, times } => {
            for _ in 0..times {
                println!("Wassup {name} :coolemoji:");
            }
        }
        Commands::Count { path } => {
            let content = std::fs::read_to_string(&path)?;
            println!("{} lines", content.lines().count());
        }
    }

    Ok(())
}
