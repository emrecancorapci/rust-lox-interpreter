use std::{ fs, iter::Peekable, str::Chars };

use token::{ Token, TokenType };
use tokenizer_error::TokenizerError;

mod tokenizer_error;
mod token;

pub struct Tokenizer {
    errors: Vec<TokenizerError>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer { errors: Vec::new(), tokens: Vec::new() }
    }
    pub fn tokenize(&mut self, filename: &str) {
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
        } else {
            println!("EOF  null");
        }
    }

    pub fn print(&self) -> i32 {
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
        let mut iterator = line.chars().into_iter().peekable();

        while let Some(ch) = iterator.next() {
            if ch == '"' {
                self.tokenize_string(&mut iterator, index);
                continue;
            }
            
            if matches!(ch,'0' ..= '9') {
                self.tokenize_number(&mut iterator, ch, index);
                continue;
            }

            if let Some(next_ch) = iterator.peek() {
                let peeked = format!("{}{}", ch, next_ch);

                if peeked.as_str() == "//" {
                    return;
                }

                let token_type = TokenType::from_two(&peeked);

                if token_type != TokenType::None {
                    self.tokens.push(Token::new_punctuator(token_type));
                    iterator.next();
                    continue;
                }
            }

            self.tokenize_char(ch, index);
        }
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

    fn tokenize_string(&mut self, iterator: &mut Peekable<Chars>, index: usize) {
        let mut string = String::new();

        loop {
            match iterator.next() {
                Some('"') => {
                    self.tokens.push(Token::new(TokenType::String, &string, &string));
                    return;
                }
                Some(string_ch) => {
                    string.push(string_ch);
                }
                None => {
                    self.errors.push(TokenizerError::unterminated_string(index + 1));
                    return;
                }
            }
        }
    }
    
    fn tokenize_number(&mut self, iterator: &mut Peekable<Chars>, ch: char, index: usize) {
        let mut string = ch.to_string();

        while let Some(num) = iterator.next() {
            match num {
                '0' ..= '9' | '.' => {
                    string.push(num);
                }
                _ => {
                    self.errors.push(TokenizerError::unexpected_char(num, index + 1));
                    return;
                }
            }
        }

        if !string.contains(".") {
            string.push_str(".0")
        }

        self.tokens.push(Token::new(TokenType::String, &string, &string));
    }
}
