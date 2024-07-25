use std::{ fs, iter::Peekable, str::Chars };

use parser_error::ParserError;
use syntax_tree::{ Binary, Expression };

mod parser_error;
mod syntax_tree;

pub struct Parser {
    errors: Vec<ParserError>,
    expression: Expression,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            errors: Vec::new(),
            expression: Expression::None,
        }
    }
    pub(crate) fn parse_file(&mut self, filename: &str) {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {}", filename);
            String::new()
        });

        if !file_contents.is_empty() {
            file_contents
                .lines()
                .enumerate()
                .for_each(|(line_index, line)| {
                    let mut iterator = line.chars().peekable();

                    match Self::parse(line_index, &mut iterator) {
                        Ok(expression) => self.expression.add(expression),
                        Err(err) => self.errors.push(err),
                    }
                });
        }
    }

    pub(crate) fn print(&self) {
        if self.errors.is_empty() {
            println!("{}", self.expression.to_string())
        } else {
            self.errors.iter().for_each(|e| e.print());
        }
    }

    fn parse(index: usize, iterator: &mut Peekable<Chars<'_>>) -> Result<Expression, ParserError> {
        let mut expression = Expression::None;

        while let Some(ch) = iterator.peek() {
            let mut answer = match ch {
                '(' =>
                    match Self::parse_group(iterator, index) {
                        Ok(expr) => expr,
                        Err(err) => {
                            return Err(err);
                        }
                    }
                '"' => Self::parse_string(iterator, index),
                '0'..='9' => Self::parse_number(iterator),
                'a'..='z' | 'A'..='Z' | '_' => Self::parse_text(iterator),
                _ => {
                    Self::parse_binary(iterator, &mut expression);
                    continue;
                }
            };

            expression.add(answer);
        }

        return Ok(expression);
    }

    fn parse_text(iterator: &mut Peekable<Chars<'_>>) -> Expression {
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

        return match string.as_str() {
            "true" => Expression::True,
            "false" => Expression::False,
            "nil" => Expression::Nil,
            _ => Expression::None,
        };
    }

    fn parse_number(iterator: &mut Peekable<Chars<'_>>) -> Expression {
        let mut number = String::new();

        while let Some(ch) = iterator.peek() {
            match ch {
                '0'..='9' => {
                    number.push(*ch);

                    iterator.next();
                }
                '.' => {
                    if number.contains('.') {
                        break;
                    } else {
                        number.push(*ch);
                        iterator.next();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if !number.contains('.') {
            number.push_str(".0");
        } else if number.ends_with('.') {
            number.push('0');
        }

        return Expression::Number(number);
    }

    fn parse_string(iterator: &mut Peekable<Chars<'_>>, _: usize) -> Expression {
        let _ = iterator.next();
        let mut string = String::new();

        loop {
            match iterator.next() {
                Some('"') => {
                    return Expression::String(string);
                }
                Some(ch) => {
                    string.push(ch);
                }
                None => {
                    return Expression::None;
                }
            }
        }
    }

    fn parse_binary(iterator: &mut Peekable<Chars<'_>>, expression: &mut Expression) {
        let ch = iterator.next().unwrap();

        let peek = iterator.peek().unwrap();

        let binary_double = Binary::create_expression_double(
            format!("{}{}", ch, peek).as_str(),
            expression.clone()
        );

        match binary_double {
            Expression::Binary(_) => {
                *expression = binary_double;
                iterator.next();
            }
            Expression::None => {
                let binary_single = Binary::create_expression_single(ch, expression.clone());

                match binary_single {
                    Expression::Binary(_) => {
                        *expression = binary_single;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn parse_group(
        iterator: &mut Peekable<Chars<'_>>,
        index: usize
    ) -> Result<Expression, ParserError> {
        iterator.next();

        let mut line = String::new();

        loop {
            match iterator.next() {
                Some(')') => {
                    break;
                }
                Some(ch) => {
                    line.push(ch);
                }
                None => {
                    return Err(ParserError::unmatched_paranthesis())
                }
            }
        }

        let mut new_iterator = line.chars().peekable();

        match Self::parse(index, &mut new_iterator) {
            Ok(expr) => Ok(Expression::Grouping(Box::new(expr))),
            Err(err) => Err(err),
        }
    }
}
