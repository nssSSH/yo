use clap::Parser;
use colored::{Colorize};
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
    let output = match args.color.to_lowercase().as_str() {
        "red" => args.message.red(),
        "blue" => args.message.blue(),
        "yellow" => args.message.yellow(),
        "green" => args.message.green(),
        "cyan" => args.message.cyan(),
        "purple" => args.message.purple(),
        "magenta" => args.message.magenta(),
        &_ => args.message.white()
    };

    println!("{output}");

}