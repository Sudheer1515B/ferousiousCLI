use clap::{Parser, Subcommand};
//parser is for parsing through those args i giv to the CLI
//subcommand is for adding onto those main arguments
// pathbuf to store filesystem paths
use std::path::PathBuf;

#[derive(Parser)]
//this is basically to gen the code needed to facilitate
// argument parshing for the CLI
#[command(
    name = "clitoolthing",
    version,
    about = "Trial to see how CLI tools work"
)]
//will run it like "clitoolthing --version"
// or "clitoolthing --nobake"
struct Cli {
    //struct which the CLI's input args are stored in
    #[command(subcommand)]
    command: Commands,
    //command variable is of type "Commands" (ie the enum below,
    // can add other commands when required by editing the enum)
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
    //using std error here to facilitate error types
    //
    let cli = Cli::parse();
    // reads the arguments given to the terminal and checks
    // if it matches the provided struct, and if yes, converts
    // them into the types in the Struct
    // Finally returns a filled out Struct

    match cli.command {
        //match here to check the command from the enum that
        // is used in the given input
        Commands::Greet { name, times } => {
            //loopin as many times as asked in the greet
            // command, default is 1 time
            for _ in 0..times {
                println!("Wassup {name} :coolemoji:");
            }
        }
        Commands::Count { path } => {
            let content = std::fs::read_to_string(&path)?;
            //RHS attempts to load a file into a string,
            // reference of the path is used so that ownership
            // isnt transferred, '?' for handling return
            println!("{} lines", content.lines().count());
        }
    }

    Ok(())
}
