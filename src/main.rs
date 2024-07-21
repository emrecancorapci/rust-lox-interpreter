use std::{env, process};
use std::io::{ self, Write };

mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let mut tokenizer = tokenizer::Tokenizer::new();
            tokenizer.tokenize(filename);

            let result = tokenizer.print();

            process::exit(result);
        }
        _ => {
            let _ = writeln!(io::stderr(), "Unknown command: {}", command);
            return;
        }
    };

}
