use super::{Token, TokenType, Tokenizer, TokenizerError};

#[derive(PartialEq)]
enum TokenizerMode {
    String,
    Number(bool),
    Identifier,
    None,
}

impl Tokenizer {
    pub(super) fn tokenize_line(
        tokens: &mut Vec<Token>,
        errors: &mut Vec<TokenizerError>,
        index: usize,
        line: &str,
    ) {
        let mut iter = line.chars().enumerate().peekable();
        let mut buffer = String::new();
        let mut mode = TokenizerMode::None;

        while let Some((_, ch)) = iter.peek() {
            match mode {
                TokenizerMode::None => {
                    let (_, ch) = iter.next().unwrap();

                    mode = match ch {
                        '"' => TokenizerMode::String,
                        '0'..='9' => {
                            buffer.push(ch);
                            TokenizerMode::Number(false)
                        }
                        'a'..='z' | 'A'..='Z' | '_' => {
                            buffer.push(ch);
                            TokenizerMode::Identifier
                        }
                        '/' => {
                            if matches!(iter.peek(), Some((_, '/'))) {
                                return;
                            } else {
                                tokens.push(Token::new_punctuator(TokenType::Slash));
                                TokenizerMode::None
                            }
                        }
                        '=' if tokens.len() > 1 => {
                            let token = tokens.pop().unwrap();

                            match token.get_type() {
                                &TokenType::Bang => {
                                    tokens.push(Token::new_punctuator(TokenType::BangEqual));
                                }
                                &TokenType::Equal => {
                                    tokens.push(Token::new_punctuator(TokenType::EqualEqual));
                                }
                                &TokenType::Greater => {
                                    tokens.push(Token::new_punctuator(TokenType::GreaterEqual));
                                }
                                &TokenType::Less => {
                                    tokens.push(Token::new_punctuator(TokenType::LessEqual));
                                }
                                _ => {
                                    tokens.push(token);
                                    tokens.push(Token::new_punctuator(TokenType::Equal));
                                }
                            }

                            TokenizerMode::None
                        }
                        _ => match TokenType::from_one(&ch) {
                            TokenType::None => {
                                errors.push(TokenizerError::unexpected_char(ch, index + 1));
                                TokenizerMode::None
                            }
                            token => {
                                tokens.push(Token::new_punctuator(token));
                                TokenizerMode::None
                            }
                        },
                    };
                }
                TokenizerMode::String => {
                    let (_, ch) = iter.next().unwrap();

                    match ch {
                        '"' => {
                            tokens.push(Token::new(
                                TokenType::String,
                                &buffer,
                                format!("\"{}\"", buffer).as_str(),
                            ));

                            buffer.clear();
                            mode = TokenizerMode::None;
                        }
                        _ => buffer.push(ch),
                    }
                }
                TokenizerMode::Number(false) => match ch {
                    '0'..='9' => {
                        buffer.push(iter.next().unwrap().1);
                    }
                    '.' => {
                        buffer.push(iter.next().unwrap().1);
                        mode = TokenizerMode::Number(true);
                    }
                    _ => {
                        tokens.push(Token::new(
                            TokenType::Number,
                            buffer.as_str(),
                            format!("{buffer}.0").as_str(),
                        ));

                        buffer.clear();
                        mode = TokenizerMode::None;
                    }
                },
                TokenizerMode::Number(true) => match ch {
                    '0'..='9' => {
                        let (_, ch) = iter.next().unwrap();

                        buffer.push(ch);
                    }
                    _ if buffer.ends_with('.') => {
                        tokens.push(Token::new(
                            TokenType::Number,
                            &buffer[..buffer.len() - 1],
                            &format!("{buffer}0"),
                        ));
                        tokens.push(Token::new_punctuator(TokenType::Dot));

                        buffer.clear();
                        mode = TokenizerMode::None;
                    }
                    _ => {
                        let mut literal = buffer.to_string();

                        while literal.ends_with('0') {
                            literal.pop();
                        }
                        if literal.ends_with('.') {
                            literal.push('0');
                        }

                        tokens.push(Token::new(
                            TokenType::Number,
                            buffer.as_str(),
                            literal.as_str(),
                        ));

                        buffer.clear();
                        mode = TokenizerMode::None;
                    }
                },
                TokenizerMode::Identifier => match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                        buffer.push(iter.next().unwrap().1);
                    }
                    _ => {
                        match TokenType::from_string(&buffer) {
                            TokenType::String => {
                                tokens.push(Token::new_identifier(&buffer));
                            }
                            token_type => {
                                tokens.push(Token::new_reserved(token_type));
                            }
                        }

                        buffer.clear();
                        mode = TokenizerMode::None;
                    }
                },
            }
        }
        match mode {
            TokenizerMode::String => errors.push(TokenizerError::unterminated_string(index + 1)),
            TokenizerMode::Number(false) => {
                tokens.push(Token::new(
                    TokenType::Number,
                    buffer.as_str(),
                    format!("{buffer}.0").as_str(),
                ));
            }
            TokenizerMode::Number(true) if buffer.ends_with('.') => {
                tokens.push(Token::new(
                    TokenType::Number,
                    &buffer[..buffer.len() - 1],
                    &format!("{buffer}0"),
                ));
                tokens.push(Token::new_punctuator(TokenType::Dot));
            }
            TokenizerMode::Number(true) => {
                let mut literal = buffer.to_string();

                while literal.ends_with('0') {
                    literal.pop();
                }
                if literal.ends_with('.') {
                    literal.push('0');
                }

                tokens.push(Token::new(
                    TokenType::Number,
                    buffer.as_str(),
                    literal.as_str(),
                ));
            }
            TokenizerMode::Identifier => match TokenType::from_string(&buffer) {
                TokenType::String => {
                    tokens.push(Token::new_identifier(&buffer));
                }
                token_type => {
                    tokens.push(Token::new_reserved(token_type));
                }
            },
            TokenizerMode::None => {}
        }
    }
}
