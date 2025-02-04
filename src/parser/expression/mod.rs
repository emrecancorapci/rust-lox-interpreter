use binary::Binary;
use unary::Unary;

pub mod binary;
pub mod unary;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Grouping(Box<Expression>),
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Number(String),
    String(String),
    True,
    False,
    Nil,
    None,
}

impl Expression {
    pub fn is_none(&self) -> bool {
        match self {
            Expression::None => true,
            _ => false,
        }
    }

    pub fn has_slot(&self) -> bool {
        match self {
            Expression::Grouping(e) => e.has_slot(),
            Expression::Binary(b) => b.has_slot(),
            Expression::Unary(u) => u.has_slot(),
            Expression::None => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Expression::Binary(_) => true,
            _ => false,
        }
    }

    pub fn is_binary_and_not_full(&self) -> bool {
        match self {
            Expression::Binary(b) => !b.is_full(),
            _ => false,
        }
    }
}

impl AddExpr for Expression {
    fn add_expr(&self, expr: Expression) -> AddExprResult {
        match self {
            Expression::Binary(binary) => binary.add_expr(expr),
            Expression::Unary(unary) => unary.add_expr(expr),
            Expression::Grouping(self_expr) => match expr {
                Expression::Binary(binary) => binary.add_expr(self.clone()),
                _ => match self_expr.add_expr(expr) {
                    AddExprResult::Done(self_expr) => {
                        AddExprResult::Done(Expression::Grouping(Box::new(self_expr)))
                    }
                    result => result,
                },
            },
            Expression::Number(ref num) => match expr {
                Expression::Binary(binary) => binary.add_expr(Expression::Number(num.to_string())),
                Expression::Unary(unary) => unary.add_expr(Expression::Number(num.to_string())),
                _ => AddExprResult::Full,
            },
            Expression::String(ref str) => match expr {
                Expression::Binary(binary) => binary.add_expr(Expression::Number(str.to_string())),
                Expression::Unary(unary) => unary.add_expr(Expression::Number(str.to_string())),
                _ => AddExprResult::Full,
            },
            Expression::True => AddExprResult::Full,
            Expression::False => AddExprResult::Full,
            Expression::Nil => AddExprResult::Full,
            Expression::None => AddExprResult::Done(expr),
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

pub enum AddExprResult {
    Done(Expression),
    Error(String),
    Full,
}

pub trait AddExpr {
    fn add_expr(&self, expr: Expression) -> AddExprResult;
}
