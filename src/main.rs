#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all
)]
#![allow(clippy::needless_return)]
#![forbid(unsafe_code)]

use std::{env, process};
use std::io::{ self, Write };

mod tokenizer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let mut tokenizer = tokenizer::Tokenizer::new();
            tokenizer.tokenize_file(filename);

            let result = tokenizer.print();

            process::exit(result);
        }
        "parse" => {
            let mut parser = parser::Parser::new();
            parser.parse_file(filename);
        }
        _ => {
            let _ = writeln!(io::stderr(), "Unknown command: {}", command);
            return;
        }
    };

}
