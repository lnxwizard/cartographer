use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "A command-line tool built with Rust for generating and saving Minecraft seeds.")]
#[command(
    long_about = "Generate random Minecraft seeds and save favorite seeds with `cartographer`."
)]
#[command(author, version)]
pub struct Cli {
    #[arg(help = "Toggle colored output")]
    #[arg(short = 'c', long = "colored")]
    pub colored: bool,
}
