mod cli;

use clap::Parser;
use cli::Cli;
use colored::Colorize;
use rand::random;

fn main() -> Result<(), ()> {
    // parse arguments
    let cli = Cli::parse();

    // generate random 64-bit signed integers
    let seed64: i64 = random();

    if !cli.color {
        println!("{}: {}", "64-bit Seed".bold().green(), seed64);
        return Ok(());
    }

    println!("{}: {}", "64-bit Seed", seed64);

    Ok(())
}
