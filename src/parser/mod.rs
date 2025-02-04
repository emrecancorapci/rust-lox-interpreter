use expression::{binary::Binary, unary::Unary, AddExpr, AddExprResult, Expression};

use crate::tokenizer::{token::Token, token_type::TokenType};

pub mod expression;
#[cfg(test)]
mod tests;

pub struct Parser {}

impl Parser {
    pub fn parse_tokens(tokens: &Vec<Token>) -> Expression {
        let mut expr = Expression::None;
        let mut groups: Vec<Expression> = Vec::new();

        for token in tokens.iter() {
            let expr_base = if groups.is_empty() {
                dbg!(&expr)
            } else {
                dbg!(groups.last().unwrap())
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
                    None => AddExprResult::Error("No left paranthesis".to_string()),
                },
                TokenType::Minus => match expr {
                    Expression::Binary(ref binary) if binary.is_full() => {
                        let add = Expression::Unary(Box::new(Unary::Minus(Expression::None)));

                        expr_base.add_expr(add)
                    }
                    _ => {
                        let add = Expression::Binary(Box::new(Binary::Minus(
                            Expression::None,
                            Expression::None,
                        )));
                        expr_base.add_expr(add)
                    }
                },
                TokenType::Plus => {
                    let add = Expression::Binary(Box::new(Binary::Plus(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::Slash => {
                    let add = Expression::Binary(Box::new(Binary::Slash(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::Star => {
                    let add = Expression::Binary(Box::new(Binary::Star(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::BangEqual => {
                    let add = Expression::Binary(Box::new(Binary::BangEqual(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::EqualEqual => {
                    let add = Expression::Binary(Box::new(Binary::EqualEqual(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::Greater => {
                    let add = Expression::Binary(Box::new(Binary::Greater(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::GreaterEqual => {
                    let add = Expression::Binary(Box::new(Binary::GreaterEqual(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::Less => {
                    let add = Expression::Binary(Box::new(Binary::Less(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                TokenType::LessEqual => {
                    let add = Expression::Binary(Box::new(Binary::LessEqual(
                        Expression::None,
                        Expression::None,
                    )));

                    expr_base.add_expr(add)
                }
                // Unary
                TokenType::Bang => {
                    let add = Expression::Unary(Box::new(Unary::Bang(Expression::None)));

                    expr_base.add_expr(add)
                }
                // Values
                TokenType::Number => {
                    let add = Expression::Number(token.get_literal().to_string());

                    expr_base.add_expr(add)
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
                AddExprResult::Done(expression) => {
                    if groups.is_empty() {
                        expr = expression
                    } else {
                        groups.pop();
                        groups.push(expression);
                    }
                }
                AddExprResult::Error(err) => panic!("{err}"),
                AddExprResult::Full => todo!(),
            }
        }

        return expr;
    }
}
