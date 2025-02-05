use interpreter_starter_rust::{parser::Parser, tokenizer::Tokenizer};

const CODE_SUCCESS: i32 = 0;
const CODE_ERROR: i32 = 65;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "This command needs at least two arguments. Usage: {} tokenize <filename>",
                args[0]
            ),
        ));
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = std::fs::read_to_string(filename)?;

    let result = match command.as_str() {
        "tokenize" => {
            let output = Tokenizer::tokenize(file_contents)?;
            Tokenizer::serialize(output.get_tokens(), output.get_errors())
        }
        "parse" => {
            let output = Tokenizer::tokenize(file_contents)?;
            let expressions = Parser::parse_tokens(output.get_tokens())?;

            println!("{expressions}");
            Ok(())
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Unknown command: {}", command),
        )),
    };

    if result.is_ok() {
        std::process::exit(CODE_SUCCESS)
    } else {
        std::process::exit(CODE_ERROR)
    }
}
