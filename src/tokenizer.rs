use std::{ fs, iter::Peekable, ops::ControlFlow, str::Chars };

use token::Token;
use token_type::TokenType;
use tokenizer_error::TokenizerError;

pub(crate) mod token_type;
mod tokenizer_error;
mod token;

pub(crate) struct Tokenizer {
    errors: Vec<TokenizerError>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub(crate) fn new() -> Tokenizer {
        Tokenizer { errors: Vec::new(), tokens: Vec::new() }
    }
    pub(crate) fn tokenize_file(&mut self, filename: &str) {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {}", filename);
            String::new()
        });

        if !file_contents.is_empty() {
            file_contents
            .lines()
            .enumerate()
            .for_each(|(line_index, line)| {
                self.tokenize_line(line_index, line);
            });
        }
    }

    pub(crate) fn print(&self) -> i32 {
        self.errors.iter().for_each(|err| { err.print() });
        self.tokens.iter().for_each(|tkn| {
            tkn.print();
        });

        println!("EOF  null");

        if self.errors.is_empty() {
            0
        } else {
            65
        }
    }

    fn tokenize_line(&mut self, index: usize, line: &str) {
        let mut iterator = line.chars().peekable();

        while let Some(ch) = iterator.peek() {
            match ch {
                '"' => self.tokenize_string(&mut iterator, index),
                '0'..='9' => self.tokenize_number(&mut iterator),
                'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier(&mut iterator),
                _ => if self.tokenize_characters(&mut iterator, index).is_break() {
                    return;
                }
            }
        }
    }

    fn tokenize_characters(
        &mut self,
        iterator: &mut Peekable<Chars<'_>>,
        index: usize
    ) -> ControlFlow<()> {
        let ch = iterator.next().unwrap();

        if let Some(next_ch) = iterator.peek() {
            let peeked = format!("{}{}", ch, next_ch);

            if peeked.as_str() == "//" {
                return ControlFlow::Break(());
            }

            let token_type = TokenType::from_two(&peeked);

            if token_type != TokenType::None {
                self.tokens.push(Token::new_punctuator(token_type));
                iterator.next();
                return ControlFlow::Continue(());
            }
        }

        self.tokenize_char(ch, index);
        return ControlFlow::Continue(());
    }

    fn tokenize_char(&mut self, ch: char, index: usize) {
        let token_type = TokenType::from_one(&ch);

        if token_type == TokenType::Tab || token_type == TokenType::Whitespace {
        } else if token_type == TokenType::None {
            self.errors.push(TokenizerError::unexpected_char(ch, index + 1));
        } else {
            self.tokens.push(Token::new_punctuator(token_type));
        }
    }

    fn tokenize_string(&mut self, iterator: &mut Peekable<Chars<'_>>, index: usize) {
        let _ = iterator.next();
        let mut literal = String::new();

        loop {
            match iterator.next() {
                Some('"') => {
                    let string = format!("\"{literal}\"");

                    self.tokens.push(Token::new(TokenType::String, &string, &literal));
                    return;
                }
                Some(ch) => {
                    literal.push(ch);
                }
                None => {
                    self.errors.push(TokenizerError::unterminated_string(index + 1));
                    return;
                }
            }
        }
    }

    fn tokenize_number(&mut self, iterator: &mut Peekable<Chars<'_>>) {
        let mut string = String::new();

        while let Some(ch) = iterator.peek() {
            match ch {
                '0'..='9' => {
                    string.push(*ch);

                    iterator.next();
                }
                '.' => {
                    if string.contains('.') {
                        break;
                    } else {
                        string.push(*ch);

                        iterator.next();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if string.contains('.') {
            if string.ends_with('.') {
                let _ = string.pop();
                let literal = format!("{}.0", string);

                self.tokens.push(Token::new(TokenType::Number, &string, &literal));
                self.tokens.push(Token::new_punctuator(TokenType::Dot));
            } else {
                let mut literal = string.clone();

                while literal.ends_with('0') {
                    literal.pop();
                }

                if literal.ends_with('.') {
                    literal.push('0');
                }

                self.tokens.push(Token::new(TokenType::Number, &string, &literal));
            }
        } else {
            let literal = format!("{}.0", string);

            self.tokens.push(Token::new(TokenType::Number, &string, &literal));
        }
    }

    fn tokenize_identifier(&mut self, iterator: &mut Peekable<Chars<'_>>) {
        let mut string = String::new();

        while let Some(ch) = iterator.peek() {
            if matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                string.push(*ch);
                iterator.next();
                continue;
            } else {
                break;
            }
        }

        match TokenType::from_string(string.as_str()) {
            TokenType::String => {
                self.tokens.push(Token::new_identifier(&string));
            }
            token_type => {
                self.tokens.push(Token::new_reserved(token_type));
            }
        }
    }
}
