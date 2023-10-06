use clap::Parser;

#[derive(Parser)]
#[command(name = "cartographer", bin_name = "crt")]
#[command(about = "A command-line tool built with Rust for generating and saving Minecraft seeds.")]
#[command(
    long_about = "Generate random Minecraft seeds and save favorite seeds with `cartographer`."
)]
#[command(author, version)]
pub struct Cli {
    #[arg(help = "Toggle colored output")]
    #[arg(short = 'c', long = "color")]
    pub color: bool,
}
