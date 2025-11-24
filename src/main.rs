mod lexer;

use std::fs;

use clap::Parser;
// use regex::Regex;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input_path: String,
}

fn main() {
    let cli = Cli::parse();

    let input = fs::read_to_string(&cli.input_path).expect("Should be able to read file");

    let _tokens = lexer::lexer(&input);
}
