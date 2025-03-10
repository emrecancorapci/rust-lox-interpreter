use crate::tokenizer::Token;

pub enum ParseExprErrorType {
    EmptyToken,
    
}

pub struct ParseExprError {
    token: Token,
    msg: String,
}

impl ParseExprError {
    fn new(token: Token, msg: String) -> Self {
        Self { token, msg }
    }
}

impl std::fmt::Display for ParseExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line {}]: Error at {}: {}",
            self.token.get_line(),
            self.token.get_literal(),
            self.msg
        )
    }
}
