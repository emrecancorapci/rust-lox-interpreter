
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

    pub fn from_string(string: &str) -> Self {
        match string {
            "and" => Self::And,
            "class" => Self::Class,
            "else" => Self::Else,
            "false" => Self::False,
            "fun" => Self::Fun,
            "for" => Self::For,
            "if" => Self::If,
            "nil" => Self::Nil,
            "or" => Self::Or,
            "print" => Self::Print,
            "return" => Self::Return,
            "super" => Self::Super,
            "this" => Self::This,
            "true" => Self::True,
            "var" => Self::Var,
            "while" => Self::While,
            _ => Self::String
        }
    }

    pub fn get_type_string(&self) -> String {
        (
            match self {
                Self::LeftParenthesis => "LEFT_PAREN",
                Self::RightParenthesis => "RIGHT_PAREN",
                Self::LeftCurly => "LEFT_BRACE",
                Self::RightCurly => "RIGHT_BRACE",
                Self::Comma => "COMMA",
                Self::Dot => "DOT",
                Self::Minus => "MINUS",
                Self::Plus => "PLUS",
                Self::Semicolon => "SEMICOLON",
                Self::Slash => "SLASH",
                Self::Star => "STAR",
                Self::Bang => "BANG",
                Self::BangEqual => "BANG_EQUAL",
                Self::Equal => "EQUAL",
                Self::EqualEqual => "EQUAL_EQUAL",
                Self::Greater => "GREATER",
                Self::GreaterEqual => "GREATER_EQUAL",
                Self::Less => "LESS",
                Self::LessEqual => "LESS_EQUAL",
                Self::Identifier => "IDENTIFIER",
                Self::String => "STRING",
                Self::Number => "NUMBER",
                Self::And => "AND",
                Self::Class => "CLASS",
                Self::Else => "ELSE",
                Self::False => "FALSE",
                Self::Fun => "FUN",
                Self::For => "FOR",
                Self::If => "IF",
                Self::Nil => "NIL",
                Self::Or => "OR",
                Self::Print => "PRINT",
                Self::Return => "RETURN",
                Self::Super => "SUPER",
                Self::This => "THIS",
                Self::True => "TRUE",
                Self::Var => "VAR",
                Self::While => "WHILE",
                Self::EOF => "EOF",
                _ => "",
            }
        ).to_string()
    }

    pub fn get_lexeme(&self) -> String {
        (
            match self {
                Self::LeftParenthesis => "(",
                Self::RightParenthesis => ")",
                Self::LeftCurly => "{",
                Self::RightCurly => "}",
                Self::Comma => ",",
                Self::Dot => ".",
                Self::Minus => "-",
                Self::Plus => "+",
                Self::Semicolon => ";",
                Self::Slash => "/",
                Self::Star => "*",
                Self::Bang => "!",
                Self::BangEqual => "!=",
                Self::Equal => "=",
                Self::EqualEqual => "==",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
                Self::Less => "<",
                Self::LessEqual => "<=",
                Self::And => "and",
                Self::Class => "class",
                Self::Else => "else",
                Self::False => "false",
                Self::Fun => "fun",
                Self::For => "for",
                Self::If => "if",
                Self::Nil => "nil",
                Self::Or => "or",
                Self::Print => "print",
                Self::Return => "return",
                Self::Super => "super",
                Self::This => "this",
                Self::True => "true",
                Self::Var => "var",
                Self::While => "while",
                _ => "",
            }
        ).to_string()
    }
}
