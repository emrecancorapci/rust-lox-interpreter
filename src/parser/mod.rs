use expression::{
    binary::{Binary, BinaryType},
    unary::Unary,
    AddExpr, Expression, IsPartial,
};

use crate::tokenizer::{token::Token, token_type::TokenType};

pub mod expression;
mod tests;

pub struct Parser {}

impl Parser {
    pub fn parse_tokens(tokens: &Vec<Token>) -> Expression {
        let mut expr = Expression::None;
        let mut groups: Vec<Expression> = Vec::new();
        let tokens = tokens.iter().filter(Parser::token_filter());

        for token in tokens {
            let expr_base = if groups.is_empty() {
                &expr
            } else {
                groups.last().unwrap()
            };

            let result = match token.get_type() {
                TokenType::LeftParenthesis => {
                    groups.push(Expression::None);
                    continue;
                }
                TokenType::RightParenthesis => match groups.pop() {
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
                    None => Err("No left paranthesis".to_string()),
                },
                TokenType::Minus => match expr_base {
                    Expression::Binary(binary) if binary.is_partial() => {
                        binary.add_expr(Unary::new_minus_expr(Expression::None))
                    }
                    Expression::None => Ok(Unary::new_minus_expr(Expression::None)),
                    _ => expr_base.add_expr(Binary::new_empty_expr(BinaryType::Minus)),
                },
                TokenType::Plus
                | TokenType::Slash
                | TokenType::Star
                | TokenType::BangEqual
                | TokenType::EqualEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    expr_base.add_expr(Parser::new_binary_from_token(token.get_type()))
                }

                // Unary
                TokenType::Bang => expr_base.add_expr(Unary::new_bang_expr(Expression::None)),
                // Values
                TokenType::Number => {
                    expr_base.add_expr(Expression::Number(token.get_literal().to_string()))
                }
                TokenType::String => {
                    let add = Expression::String(token.get_literal().to_string());

                    expr_base.add_expr(add)
                }
                TokenType::True => {
                    let add = Expression::True;

                    expr_base.add_expr(add)
                }
                TokenType::False => {
                    let add = Expression::False;

                    expr_base.add_expr(add)
                }
                TokenType::Nil => {
                    let add = Expression::Nil;

                    expr_base.add_expr(add)
                }
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

        return expr;
    }

    fn new_binary_from_token(token: &TokenType) -> Expression {
        match token {
            TokenType::Plus => Binary::new_empty_expr(BinaryType::Plus),
            TokenType::Slash => Binary::new_empty_expr(BinaryType::Slash),
            TokenType::Star => Binary::new_empty_expr(BinaryType::Star),
            TokenType::BangEqual => Binary::new_empty_expr(BinaryType::BangEqual),
            TokenType::EqualEqual => Binary::new_empty_expr(BinaryType::EqualEqual),
            TokenType::Greater => Binary::new_empty_expr(BinaryType::Greater),
            TokenType::GreaterEqual => Binary::new_empty_expr(BinaryType::GreaterEqual),
            TokenType::Less => Binary::new_empty_expr(BinaryType::Less),
            TokenType::LessEqual => Binary::new_empty_expr(BinaryType::LessEqual),
            _ => panic!("Wrong use"),
        }
    }

    fn token_filter() -> impl FnMut(&&Token) -> bool {
        |t: &&Token| {
            matches!(
                t.get_type(),
                TokenType::LeftParenthesis
                    | TokenType::RightParenthesis
                    | TokenType::Bang
                    | TokenType::BangEqual
                    | TokenType::Equal
                    | TokenType::EqualEqual
                    | TokenType::False
                    | TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
                    | TokenType::Minus
                    | TokenType::Nil
                    | TokenType::Number
                    | TokenType::Plus
                    | TokenType::Slash
                    | TokenType::Star
                    | TokenType::String
                    | TokenType::True
            )
        }
    }
}
