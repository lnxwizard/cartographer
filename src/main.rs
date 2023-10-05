mod cli;

use clap::Parser;
use cli::Cli;
use rand::random;

fn main() {
    let _cli = Cli::parse();

    let seed64: i64 = random();
    let seed32: i32 = random();

    println!("64-bit Seed: {}", seed64);
    println!("32-bit Seed: {}", seed32);
}
