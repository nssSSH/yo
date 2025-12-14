use clap::{Args, Parser};
use colored::{Colorize, ColoredString};
/// Simple program to display a colored message on startup of terminal
#[derive(Parser, Debug)]
#[command(version, about)]
struct Init {
    /// The message to be displayed on startup of terminal
    #[arg(short, long)]
    message: String,
    /// The color of the message to be displayed on startup of terminal
    #[arg(short, long)]
    color: String,
}

fn main(){
    let args = Init::parse();
    println!("{:?}", args);

}