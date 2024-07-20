use std::{env, process};
use std::io::{ self, Write };

mod tokenize;

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
            let result = tokenize::tokenize(filename);
            process::exit(result);
        }
        _ => {
            let _ = writeln!(io::stderr(), "Unknown command: {}", command);
            return;
        }
    };

}
