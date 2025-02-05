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
            Expression::None => true,
            _ => false,
        }
    }
    pub(crate) fn is_binary(&self) -> bool {
        match self {
            Expression::Binary(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_binary(&self) -> Result<&Binary, String> {
        match self {
            Expression::Binary(b) => Ok(b),
            _ => todo!(),
        }
    }
}

pub trait IsPartial {
    fn is_partial(&self) -> bool;
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
            Expression::Grouping(self_expr) => match expr {
                Expression::Binary(binary) => binary.add_expr(self.clone()),
                _ => match self_expr.add_expr(expr) {
                    Ok(self_expr) => Ok(Expression::Grouping(Box::new(self_expr))),
                    result => result,
                },
            },
            Expression::Number(ref num) => match expr {
                Expression::Binary(binary) => binary.add_expr(Expression::Number(num.to_string())),
                Expression::Unary(unary) => unary.add_expr(Expression::Number(num.to_string())),
                _ => Err(todo!()),
            },
            Expression::String(ref str) => match expr {
                Expression::Binary(binary) => binary.add_expr(Expression::Number(str.to_string())),
                Expression::Unary(unary) => unary.add_expr(Expression::Number(str.to_string())),
                _ => Err(todo!()),
            },
            Expression::True => Err(todo!()),
            Expression::False => Err(todo!()),
            Expression::Nil => Err(todo!()),
            Expression::None => Ok(expr),
        }
    }
}
