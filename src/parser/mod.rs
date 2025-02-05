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
                    None => AddExprResult::Error("No left paranthesis".to_string()),
                },
                TokenType::Minus => match expr_base {
                    Expression::Binary(binary) if binary.has_slot() => {
                        let add = Expression::Unary(Box::new(Unary::Minus(Expression::None)));
                        binary.add_expr(add)
                    }
                    Expression::None => AddExprResult::Done(Expression::Unary(Box::new(
                        Unary::Minus(Expression::None),
                    ))),
                            _ => {
                                let add = Expression::Binary(Box::new(Binary::Minus(
                                    Expression::None,
                                    Expression::None,
                                )));
                                expr_base.add_expr(add)
                            }
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
                AddExprResult::Full => panic!("Statement was full"),
            }
        }

        return expr;
    }

    fn new_binary_from_token(token: &TokenType) -> Expression {
        match token {
            TokenType::Plus => {
                Expression::Binary(Box::new(Binary::Plus(Expression::None, Expression::None)))
            }
            TokenType::Slash => {
                Expression::Binary(Box::new(Binary::Slash(Expression::None, Expression::None)))
            }

            TokenType::Star => {
                Expression::Binary(Box::new(Binary::Star(Expression::None, Expression::None)))
            }

            TokenType::BangEqual => Expression::Binary(Box::new(Binary::BangEqual(
                Expression::None,
                Expression::None,
            ))),

            TokenType::EqualEqual => Expression::Binary(Box::new(Binary::EqualEqual(
                Expression::None,
                Expression::None,
            ))),

            TokenType::Greater => Expression::Binary(Box::new(Binary::Greater(
                Expression::None,
                Expression::None,
            ))),

            TokenType::GreaterEqual => Expression::Binary(Box::new(Binary::GreaterEqual(
                Expression::None,
                Expression::None,
            ))),

            TokenType::Less => {
                Expression::Binary(Box::new(Binary::Less(Expression::None, Expression::None)))
            }

            TokenType::LessEqual => Expression::Binary(Box::new(Binary::LessEqual(
                Expression::None,
                Expression::None,
            ))),
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
