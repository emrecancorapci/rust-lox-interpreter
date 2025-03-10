use std::{fmt::format, io::{Error, ErrorKind}};

use error::ParseExprError;
use expression::{
    binary::{Binary, BinaryType},
    unary::Unary,
    AddExpr, Expression, IsPartial,
};

use crate::tokenizer::{token::Token, token_type::TokenType};

pub mod expression;
pub mod error;
mod tests;

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Expression, ParseExprError> {
        if tokens.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Cannot parse empty token list",
            ));
        }

        let mut expr = Expression::None;
        let mut groups = Vec::new();
        let mut groups_line = Vec::new();

        for token in tokens.iter().filter(Parser::token_filter()) {
            let expr_base = groups.last().unwrap_or(&expr);

            let result = match token.get_type() {
                TokenType::LeftParenthesis => {
                    groups.push(Expression::None);
                    groups_line.push(token.get_line());
                    continue;
                }
                TokenType::RightParenthesis => match groups.pop() {
                    Some(group_expr) => {
                        if groups.is_empty() {
                            groups_line.pop();
                            expr.add_expr(Expression::Grouping(Box::new(group_expr)))
                        } else {
                            groups_line.pop();
                            groups
                                .last()
                                .unwrap()
                                .add_expr(Expression::Grouping(Box::new(group_expr)))
                        }
                    }
                    Option::None => Err(format!("[line {}] No left paranthesis", token.get_line())),
                },
                TokenType::Minus if expr_base.is_binary() && expr_base.is_partial() => expr_base
                    .get_binary()
                    .and_then(|b| b.add_expr(Unary::new_minus_expr(Expression::None))),
                TokenType::Minus if expr_base.is_none() => {
                    Ok(Unary::new_minus_expr(Expression::None))
                }
                TokenType::Minus => expr_base.add_expr(Binary::new_empty_expr(BinaryType::Minus)),
                TokenType::Bang => expr_base.add_expr(Unary::new_bang_expr(Expression::None)),
                TokenType::Number => expr_base.add_expr(Expression::Number(token.get_literal())),
                TokenType::String => expr_base.add_expr(Expression::String(token.get_literal())),
                TokenType::True => expr_base.add_expr(Expression::True),
                TokenType::False => expr_base.add_expr(Expression::False),
                TokenType::Nil => expr_base.add_expr(Expression::Nil),
                TokenType::Plus
                | TokenType::Slash
                | TokenType::Star
                | TokenType::BangEqual
                | TokenType::EqualEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => expr_base.add_expr(Parser::new_binary(token.get_type())?),
                t => {
                    eprintln!("Unknown token: {t}");
                    continue;
                }
            };

            match result {
                Ok(expression) => {
                    if groups.is_empty() {
                        expr = expression
                    } else {
                        groups.pop();
                        groups_line.pop();
                        groups.push(expression);
                    }
                }
                Err(err) => panic!("{err}"),
            }
        }

        if !groups.is_empty() {
            Err(Error::new(ErrorKind::InvalidInput, format!("")))
        } else {
            Ok(expr)
        }
    }

    fn new_binary(token: &TokenType) -> Result<Expression, Error> {
        use TokenType::*;
        match token {
            Plus | Slash | Star | BangEqual | EqualEqual | Greater | GreaterEqual | Less
            | LessEqual => Ok(Binary::new_empty_expr(BinaryType::try_from(token)?)),
            _ => Err(Error::new(
                ErrorKind::Unsupported,
                format!("Can't create binary from: {}", token),
            )),
        }
    }

    fn token_filter() -> impl FnMut(&&Token) -> bool {
        use TokenType::*;
        |t: &&Token| {
            matches!(
                t.get_type(),
                LeftParenthesis
                    | RightParenthesis
                    | Bang
                    | BangEqual
                    | Equal
                    | EqualEqual
                    | False
                    | Greater
                    | GreaterEqual
                    | Less
                    | LessEqual
                    | Minus
                    | Nil
                    | Number
                    | Plus
                    | Slash
                    | Star
                    | String
                    | True
            )
        }
    }
}

impl TryFrom<&TokenType> for BinaryType {
    type Error = Error;

    fn try_from(value: &TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Plus => Ok(BinaryType::Plus),
            TokenType::Slash => Ok(BinaryType::Slash),
            TokenType::Star => Ok(BinaryType::Star),
            TokenType::BangEqual => Ok(BinaryType::BangEqual),
            TokenType::EqualEqual => Ok(BinaryType::EqualEqual),
            TokenType::Greater => Ok(BinaryType::Greater),
            TokenType::GreaterEqual => Ok(BinaryType::GreaterEqual),
            TokenType::Less => Ok(BinaryType::Less),
            TokenType::LessEqual => Ok(BinaryType::LessEqual),
            _ => Err(Error::new(
                ErrorKind::Unsupported,
                format!("Unexpected token for BinaryType"),
            )),
        }
    }
}
