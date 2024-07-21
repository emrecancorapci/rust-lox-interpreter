pub struct TokenizerError {
    line: usize,
    error_string: String,
}

impl TokenizerError {
    pub fn new(error_string: &str, line: usize) -> Self {
        Self {
            line,
            error_string: error_string.to_string(),
        }
    }

    pub fn unexpected_char(char: char, line: usize) -> Self {
        let string = format!("Unexpected character: {}", char);

        Self::new(string.as_str(), line)
    }

    pub fn unterminated_string(line: usize) -> Self {
        Self::new("Unterminated string.", line)
    }

    pub fn print(&self) {
        eprintln!("[line {}] Error: {}", self.line, self.error_string);
    }
}
