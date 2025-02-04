use super::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    string: String,
    literal: String,
}

impl Token {
    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_literal(&self) -> &str {
        &self.literal
    }

    pub(super) fn new(token_type: TokenType, string: &str, literal: &str) -> Self {
        Self {
            token_type,
            string: string.to_string(),
            literal: literal.to_string(),
        }
    }

    pub(super) fn new_punctuator(token_type: TokenType) -> Self {
        let string = token_type.get_lexeme();

        Self {
            token_type,
            string,
            literal: "null".to_string(),
        }
    }

    pub(super) fn new_identifier(string: &str) -> Self {
        Self {
            token_type: TokenType::Identifier,
            string: string.to_string(),
            literal: "null".to_string(),
        }
    }

    pub(super) fn new_reserved(token_type: TokenType) -> Self {
        Self {
            string: token_type.get_lexeme(),
            token_type,
            literal: "null".to_string(),
        }
    }

    pub(super) fn new_eof() -> Self {
        Self {
            token_type: TokenType::EOF,
            string: "".to_string(),
            literal: "null".to_string(),
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
