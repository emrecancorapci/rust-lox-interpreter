use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    string: String,
    literal: String,
    line: usize,
}

impl Token {
    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_literal(&self) -> &str {
        &self.literal
    }

    pub fn get_line(&self) -> &usize {
        &self.line
    }

    pub(super) fn new(token_type: TokenType, string: &str, literal: &str, line: usize) -> Self {
        Self {
            token_type,
            string: string.to_string(),
            literal: literal.to_string(),
            line,
        }
    }

    pub(super) fn new_punctuator(token_type: TokenType, line: usize) -> Self {
        let string = token_type.get_lexeme();

        Self {
            token_type,
            string,
            literal: "null".to_string(),
            line,
        }
    }

    pub(super) fn new_identifier(string: &str, line: usize) -> Self {
        Self {
            token_type: TokenType::Identifier,
            string: string.to_string(),
            literal: "null".to_string(),
            line,
        }
    }

    pub(super) fn new_reserved(token_type: TokenType, line: usize) -> Self {
        Self {
            string: token_type.get_lexeme(),
            token_type,
            literal: "null".to_string(),
            line,
        }
    }

    pub(super) fn new_eof() -> Self {
        Self {
            token_type: TokenType::EOF,
            string: "".to_string(),
            literal: "null".to_string(),
            line: 0,
        }
    }

    pub(super) fn new_unknown(line: usize) -> Self {
        Self {
            token_type: TokenType::Unkonwn,
            string: "".to_string(),
            literal: "".to_string(),
            line,
        }
    }

    pub(super) fn print(&self) {
        println!(
            "{} {} {}",
            self.token_type.get_type_string(),
            self.string,
            self.literal
        )
    }
}
