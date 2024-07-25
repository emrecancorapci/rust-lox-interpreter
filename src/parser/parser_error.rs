pub(crate) struct ParserError {
    error_string: String,
}

impl ParserError {
    pub(crate) fn new(error_string: &str) -> Self {
        Self {
            error_string: error_string.to_string(),
        }
    }

    pub(crate) fn unmatched_paranthesis() -> Self {
        Self::new("Unmatched parentheses.")
    }

    pub(crate) fn print(&self) {
        eprintln!("Error: {}", self.error_string);
    }
}