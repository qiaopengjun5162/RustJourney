use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// An interesting command-line utility
#[derive(Parser, Debug)]
#[command(author = "Qiao", version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long)]
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);

    println!("Hello, world!");
}
