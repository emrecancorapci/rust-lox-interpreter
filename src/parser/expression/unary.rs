use super::{AddExpr, AddExprResult, Expression};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Unary {
    Bang(Expression),
    Minus(Expression),
}

impl Unary {
    pub fn has_slot(&self) -> bool {
        match self {
            Unary::Bang(e) => e.has_slot(),
            Unary::Minus(e) => e.has_slot(),
        }
    }
}

impl AddExpr for Unary {
    fn add_expr(&self, expr: Expression) -> AddExprResult {
        match self {
            Unary::Bang(e) if e.is_none() => {
                AddExprResult::Done(Expression::Unary(Box::new(Unary::Bang(expr))))
            }
            Unary::Minus(e) if e.is_none() => {
                AddExprResult::Done(Expression::Unary(Box::new(Unary::Minus(expr))))
            }
            Unary::Bang(e) if e.has_slot() => match e.add_expr(expr) {
                AddExprResult::Done(new_e) => {
                    AddExprResult::Done(Expression::Unary(Box::new(Unary::Bang(new_e))))
                }
                AddExprResult::Error(_) => todo!(),
                AddExprResult::Full => todo!(),
            },
            Unary::Minus(e) if e.is_none() => match e.add_expr(expr) {
                AddExprResult::Done(new_e) => {
                    AddExprResult::Done(Expression::Unary(Box::new(Unary::Minus(new_e))))
                }
                result => result,
            },
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
