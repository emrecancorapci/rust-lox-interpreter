use std::io::{Error, ErrorKind};

use expression::{
    binary::{Binary, BinaryType},
    unary::Unary,
    AddExpr, Expression, IsPartial,
};

use crate::tokenizer::{token::Token, token_type::TokenType};

pub mod expression;
mod tests;

pub struct Parser;

impl Parser {
    pub fn parse_tokens(tokens: &Vec<Token>) -> Result<Expression, Error> {
        use TokenType::*;
        if tokens.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Cannot parse empty token list",
            ));
        }

        let mut expr = Expression::None;
        let mut groups = Vec::new();

        for token in tokens.iter().filter(Parser::token_filter()) {
            let expr_base = groups.last().unwrap_or(&expr);

            let result = match token.get_type() {
                LeftParenthesis => {
                    groups.push(Expression::None);
                    continue;
                }
                RightParenthesis => match groups.pop() {
                    Some(group_expr) => {
                        if groups.is_empty() {
                            expr.add_expr(Expression::Grouping(Box::new(group_expr)))
                        } else {
                            groups
                                .last()
                                .unwrap()
                                .add_expr(Expression::Grouping(Box::new(group_expr)))
                        }
                    }
                    Option::None => Err("No left paranthesis".to_string()),
                },
                Minus => match expr_base {
                    Expression::Binary(binary) if binary.is_partial() => {
                        binary.add_expr(Unary::new_minus_expr(Expression::None))
                    }
                    Expression::None => Ok(Unary::new_minus_expr(Expression::None)),
                    _ => expr_base.add_expr(Binary::new_empty_expr(BinaryType::Minus)),
                },
                Plus | Slash | Star | BangEqual | EqualEqual | Greater | GreaterEqual | Less
                | LessEqual => expr_base.add_expr(Parser::new_binary(token.get_type())?),
                Bang => expr_base.add_expr(Unary::new_bang_expr(Expression::None)),
                Number => expr_base.add_expr(Expression::Number(token.get_literal())),
                String => expr_base.add_expr(Expression::String(token.get_literal())),
                True => expr_base.add_expr(Expression::True),
                False => expr_base.add_expr(Expression::False),
                Nil => expr_base.add_expr(Expression::Nil),
                _ => {
                    continue;
                }
            };

            match result {
                Ok(expression) => {
                    if groups.is_empty() {
                        expr = expression
                    } else {
                        groups.pop();
                        groups.push(expression);
                    }
                }
                Err(err) => panic!("{err}"),
            }
        }

        Ok(expr)
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
