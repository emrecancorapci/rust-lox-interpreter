pub struct Token {
    token_type: TokenType,
    string: String,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, string: &String, literal: &String) -> Self {
        Self {
            token_type,
            string: string.clone(),
            literal: literal.clone(),
        }
    }

    pub fn new_punctuator(token_type: TokenType) -> Self {
        let string = token_type.get_lexeme();

        Self {
            token_type,
            string,
            literal: "null".to_string(),
        }
    }

    pub fn new_identifier(string: &str) -> Self {
        Self {
            token_type: TokenType::Identifier,
            string: string.to_string(),
            literal: "null".to_string()
        }
    }

    pub fn print(&self) {
        println!("{} {} {}", self.token_type.get_type_string(), self.string, self.literal)
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParenthesis,
    RightParenthesis,
    LeftCurly,
    RightCurly,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
    None,

    // Whitespaces
    Whitespace,
    Tab
}

impl TokenType {
    pub fn from_one(ch: &char) -> Self {
        match ch {
            '(' => Self::LeftParenthesis,
            ')' => Self::RightParenthesis,
            '{' => Self::LeftCurly,
            '}' => Self::RightCurly,
            ';' => Self::Semicolon,
            ',' => Self::Comma,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Star,
            '<' => Self::Less,
            '>' => Self::Greater,
            '/' => Self::Slash,
            '.' => Self::Dot,
            '=' => Self::Equal,
            '!' => Self::Bang,
            ' ' => Self::Whitespace,
            '\u{0009}' => Self::Tab,
            _ => Self::None
        }
    }

    pub fn from_two(char_pair: &str) -> Self {
        match char_pair {
            "!=" => Self::BangEqual,
            "==" => Self::EqualEqual,
            "<=" => Self::LessEqual,
            ">=" => Self::GreaterEqual,
            _ => Self::None,
        }
    }

    pub fn get_type_string(&self) -> String {
        (
            match self {
                TokenType::LeftParenthesis => "LEFT_PAREN",
                TokenType::RightParenthesis => "RIGHT_PAREN",
                TokenType::LeftCurly => "LEFT_BRACE",
                TokenType::RightCurly => "RIGHT_BRACE",
                TokenType::Comma => "COMMA",
                TokenType::Dot => "DOT",
                TokenType::Minus => "MINUS",
                TokenType::Plus => "PLUS",
                TokenType::Semicolon => "SEMICOLON",
                TokenType::Slash => "SLASH",
                TokenType::Star => "STAR",
                TokenType::Bang => "BANG",
                TokenType::BangEqual => "BANG_EQUAL",
                TokenType::Equal => "EQUAL",
                TokenType::EqualEqual => "EQUAL_EQUAL",
                TokenType::Greater => "GREATER",
                TokenType::GreaterEqual => "GREATER_EQUAL",
                TokenType::Less => "LESS",
                TokenType::LessEqual => "LESS_EQUAL",
                TokenType::Identifier => "IDENTIFIER",
                TokenType::String => "STRING",
                TokenType::Number => "NUMBER",
                TokenType::And => "AND",
                TokenType::Class => "CLASS",
                TokenType::Else => "ELSE",
                TokenType::False => "FALSE",
                TokenType::Fun => "FUN",
                TokenType::For => "FOR",
                TokenType::If => "IF",
                TokenType::Nil => "NIL",
                TokenType::Or => "OR",
                TokenType::Print => "PRINT",
                TokenType::Return => "RETURN",
                TokenType::Super => "SUPER",
                TokenType::This => "THIS",
                TokenType::True => "TRUE",
                TokenType::Var => "VAR",
                TokenType::While => "WHILE",
                TokenType::EOF => "EOF",
                _ => "",
            }
        ).to_string()
    }

    pub fn get_lexeme(&self) -> String {
        (
            match self {
                TokenType::LeftParenthesis => "(",
                TokenType::RightParenthesis => ")",
                TokenType::LeftCurly => "{",
                TokenType::RightCurly => "}",
                TokenType::Comma => ",",
                TokenType::Dot => ".",
                TokenType::Minus => "-",
                TokenType::Plus => "+",
                TokenType::Semicolon => ";",
                TokenType::Slash => "/",
                TokenType::Star => "*",
                TokenType::Bang => "!",
                TokenType::BangEqual => "!=",
                TokenType::Equal => "=",
                TokenType::EqualEqual => "==",
                TokenType::Greater => ">",
                TokenType::GreaterEqual => ">=",
                TokenType::Less => "<",
                TokenType::LessEqual => "<=",
                _ => "",
            }
        ).to_string()
    }
}
