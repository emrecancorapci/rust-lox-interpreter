use std::io::{Error, ErrorKind};

pub use token::Token;
pub use token_type::TokenType;

pub mod token;
pub mod token_type;

mod tokenize;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn tokenize(file_contents: String) -> Result<TokenizerOutput, std::io::Error> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<TokenizerError> = Vec::new();

        if !file_contents.is_empty() {
            for (index, line) in file_contents.lines().enumerate() {
                Tokenizer::tokenize_line(&mut tokens, &mut errors, index, line);
            }
        }

        tokens.push(Token::new_eof());

        Ok(TokenizerOutput { tokens, errors })
    }

    pub fn serialize(tokens: &Vec<Token>, errors: &Vec<TokenizerError>) -> Result<(), Error> {
        errors.iter().for_each(|err| err.print());
        tokens.iter().for_each(|t| {
            if !matches!(
                t.get_type(),
                TokenType::Whitespace | TokenType::Tab | TokenType::Unkonwn
            ) {
                t.print()
            }
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidData, ""))
        }
    }
}

pub struct TokenizerOutput {
    tokens: Vec<Token>,
    errors: Vec<TokenizerError>,
}

impl TokenizerOutput {
    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn filter_empty(&self) -> impl FnOnce(&&Token) -> bool {
        |t: &&Token| {
            !matches!(
                t.get_type(),
                TokenType::Whitespace | TokenType::Tab | TokenType::Unkonwn
            )
        }
    }

    pub fn get_errors(&self) -> &Vec<TokenizerError> {
        &self.errors
    }
}

pub struct TokenizerError {
    line: usize,
    error_string: String,
}

impl TokenizerError {
    pub(crate) fn new(error_string: &str, line: usize) -> Self {
        Self {
            line,
            error_string: error_string.to_string(),
        }
    }

    pub(crate) fn unexpected_char(char: char, line: usize) -> Self {
        let string = format!("Unexpected character: {}", char);

        Self::new(string.as_str(), line)
    }

    pub(crate) fn unterminated_string(line: usize) -> Self {
        Self::new("Unterminated string.", line)
    }

    pub(crate) fn print(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.error_string);
    }
}
