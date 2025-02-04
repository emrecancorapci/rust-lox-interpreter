use std::io::{Error, ErrorKind};
use std::{env, fs, process};

use interpreter_starter_rust::parser::Parser;
use interpreter_starter_rust::tokenizer::Tokenizer;

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
    let file_contents = fs::read_to_string(filename)?;

    match command.as_str() {
        "tokenize" => {
            let output = Tokenizer::tokenize(file_contents)?;
            let result = Tokenizer::serialize(output.get_tokens(), output.get_errors());

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
