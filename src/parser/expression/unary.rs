use super::{AddExpr, Expression, IsPartial};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UnaryType {
    Bang,
    Minus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unary {
    unary_type: UnaryType,
    expression: Box<Expression>,
}

impl Unary {
    fn new(unary_type: UnaryType, expr: Expression) -> Self {
        Self {
            unary_type,
            expression: Box::new(expr),
        }
    }
    pub fn new_bang(expr: Expression) -> Self {
        Self::new(UnaryType::Bang, expr)
    }
    pub fn new_bang_expr(expr: Expression) -> Expression {
        Expression::Unary(Self::new_bang(expr))
    }
    pub fn new_minus(expr: Expression) -> Self {
        Self::new(UnaryType::Minus, expr)
    }
    pub fn new_minus_expr(expr: Expression) -> Expression {
        Expression::Unary(Self::new_minus(expr))
    }
}

impl IsPartial for Unary {
    fn is_partial(&self) -> bool {
        self.expression.is_partial()
    }
}

impl AddExpr for Unary {
    fn add_expr(&self, expr: Expression) -> Result<Expression, String> {
        match self.unary_type {
            UnaryType::Bang if self.expression.is_none() => Ok(Self::new_bang_expr(expr)),
            UnaryType::Minus if self.expression.is_none() => Ok(Self::new_minus_expr(expr)),
            UnaryType::Bang if self.expression.is_partial() => {
                Ok(Self::new_bang_expr(self.expression.add_expr(expr)?))
            }
            UnaryType::Minus if self.expression.is_partial() => {
                Ok(Self::new_minus_expr(self.expression.add_expr(expr)?))
            }
            UnaryType::Bang if expr.is_binary() => {
                expr.add_expr(Unary::new_bang_expr(self.expression.as_ref().clone()))
            }
            UnaryType::Minus if expr.is_binary() => {
                expr.add_expr(Unary::new_minus_expr(self.expression.as_ref().clone()))
            }
            _ => Err("Unary is full".to_string()),
        }
    }
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.unary_type {
            UnaryType::Bang => write!(f, "(! {})", self.expression),
            UnaryType::Minus => write!(f, "(- {})", self.expression),
        }
    }
}
