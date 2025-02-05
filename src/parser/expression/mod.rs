use binary::Binary;
use unary::Unary;

pub mod binary;
pub mod unary;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Grouping(Box<Expression>),
    Binary(Binary),
    Unary(Unary),
    Number(String),
    String(String),
    True,
    False,
    Nil,
    None,
}

impl Expression {
    pub(crate) fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
    pub(crate) fn is_binary(&self) -> bool {
        match self {
            Self::Binary(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_binary(&self) -> Result<&Binary, String> {
        match self {
            Self::Binary(b) => Ok(b),
            _ => Err("The expression is not a binary".to_string()),
        }
    }

    fn is_unary(&self) -> bool {
        match self {
            Self::Unary(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_unary(&self) -> Result<&Unary, String> {
        match self {
            Self::Unary(u) => Ok(u),
            _ => Err("The expression is not a unary".to_string()),
        }
    }
}

pub trait IsPartial {
    fn is_partial(&self) -> bool;
    fn is_full(&self) -> bool {
        !self.is_partial()
    }
}

impl IsPartial for Expression {
    fn is_partial(&self) -> bool {
        match self {
            Expression::Grouping(e) => e.is_partial(),
            Expression::Binary(b) => b.is_partial(),
            Expression::Unary(u) => u.is_partial(),
            Expression::None => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Grouping(e) => write!(f, "(group {e})"),
            Expression::Binary(b) => write!(f, "{b}"),
            Expression::Unary(u) => write!(f, "{u}"),
            Expression::Number(n) => write!(f, "{n}"),
            Expression::String(s) => write!(f, "{s}"),
            Expression::True => write!(f, "true"),
            Expression::False => write!(f, "false"),
            Expression::Nil => write!(f, "nil"),
            Expression::None => write!(f, ""),
        }
    }
}

pub trait AddExpr {
    fn add_expr(&self, expr: Expression) -> Result<Expression, String>;
}

impl AddExpr for Expression {
    fn add_expr(&self, expr: Expression) -> Result<Expression, String> {
        match self {
            Expression::Binary(binary) => binary.add_expr(expr),
            Expression::Unary(unary) => unary.add_expr(expr),
            Expression::Grouping(_) if expr.is_binary() => {
                expr.get_binary()?.add_expr(self.clone())
            }
            Expression::Grouping(self_expr) => self_expr
                .add_expr(expr)
                .and_then(|new_expr| Ok(Expression::Grouping(Box::new(new_expr)))),
            Expression::Number(num) if expr.is_binary() => expr
                .get_binary()
                .and_then(|b| b.add_expr(Expression::Number(num.to_string()))),
            Expression::Number(num) if expr.is_unary() => expr
                .get_unary()
                .and_then(|u| u.add_expr(Expression::Number(num.to_string()))),
            Expression::String(num) if expr.is_binary() => expr
                .get_binary()
                .and_then(|b| b.add_expr(Expression::String(num.to_string()))),
            Expression::String(num) if expr.is_unary() => expr
                .get_unary()
                .and_then(|u| u.add_expr(Expression::String(num.to_string()))),
            Expression::None => Ok(expr),
            _ => Err("You can't add new expression to this expression.".to_string()),
        }
    }
}
