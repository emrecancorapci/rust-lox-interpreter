#![warn(missing_debug_implementations, rust_2018_idioms, clippy::all)]
#![allow(clippy::needless_return)]
#![forbid(unsafe_code)]

use std::io::{Error, ErrorKind};
use std::{env, process};

use tokenizer::Tokenizer;

mod parser;
mod tokenizer;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "This command needs at least two arguments. Usage: {} tokenize <filename>",
                args[0]
            ),
        ));
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let (tokens, errors) = Tokenizer::tokenize_file(filename)?;

            let result = Tokenizer::serialize(&tokens, &errors);

            process::exit(result);
        }
        "parse" => {
            let mut parser = parser::Parser::new();
            parser.parse_file(filename);

            parser.print();
        }
        _ => {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Unknown command: {}", command),
            ));
        }
    };

    Ok(())
}
