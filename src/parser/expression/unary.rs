use super::{AddExpr, AddExprResult, Expression};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Unary {
    Bang(Expression),
    Minus(Expression),
}

impl AddExpr for Unary {
    fn add_expr(&self, expr: Expression) -> AddExprResult {
        match self {
            Unary::Bang(expression) if expression.is_none() => {
                AddExprResult::Done(Expression::Unary(Box::new(Unary::Bang(expr))))
            }
            Unary::Minus(expression) if expression.is_none() => {
                AddExprResult::Done(Expression::Unary(Box::new(Unary::Minus(expr))))
            }
            _ => AddExprResult::Full,
        }
    }
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Bang(e) => write!(f, "(! {e})"),
            Unary::Minus(e) => write!(f, "(- {e})"),
        }
    }
}
