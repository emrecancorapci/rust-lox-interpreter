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

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Unary {
    Bang(Expression),
    Minus(Expression),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Binary {
    EqualEqual(Expression, Expression),
    BangEqual(Expression, Expression),
    Greater(Expression, Expression),
    GreaterEqual(Expression, Expression),
    Less(Expression, Expression),
    LessEqual(Expression, Expression),
    Plus(Expression, Expression),
    Minus(Expression, Expression),
    Star(Expression, Expression),
    Slash(Expression, Expression),
}

impl Expression {
    pub(super) fn add(&mut self, expr: Expression) {
        match &self.add_expr(expr) {
            AddExprResult::Expression(new) => {
                *self = new.clone();
            }
            AddExprResult::Full => {}
            AddExprResult::Error(_) => {}
            _ => {}
        }
    }
}

impl Binary {
    pub(super) fn create_expression_single(ch: char, expr: Expression) -> Expression {
        let binary = match ch {
            '+' => Binary::Plus(expr, Expression::None),
            '-' => Binary::Minus(expr, Expression::None),
            '*' => Binary::Star(expr, Expression::None),
            '/' => Binary::Slash(expr, Expression::None),
            '>' => Binary::Greater(expr, Expression::None),
            '<' => Binary::Less(expr, Expression::None),
            _ => {
                return Expression::None;
            }
        };

        Expression::Binary(Box::new(binary))
    }

    pub(super) fn create_expression_double(str: &str, expr: Expression) -> Expression {
        let binary = match str {
            "==" => Binary::EqualEqual(expr, Expression::None),
            "<=" => Binary::LessEqual(expr, Expression::None),
            ">=" => Binary::GreaterEqual(expr, Expression::None),
            "!=" => Binary::BangEqual(expr, Expression::None),
            _ => {
                return Expression::None;
            }
        };

        Expression::Binary(Box::new(binary))
    }
}

pub trait AddExpression {
    fn add_expr(&mut self, expr: Expression) -> AddExprResult;
}

impl AddExpression for Expression {
    fn add_expr(&mut self, expr: Expression) -> AddExprResult {
        match self {
            Expression::Grouping(current) => {
                match current.add_expr(expr) {
                    AddExprResult::Expression(new) =>
                        AddExprResult::Expression(Expression::Grouping(Box::new(new))),
                    AddExprResult::Full => AddExprResult::Full,
                    AddExprResult::Error(err) => AddExprResult::Error(err),
                    _ => AddExprResult::Error("Wrong type".to_string()),
                }
            }
            Expression::Binary(current) => {
                match current.add_expr(expr) {
                    AddExprResult::Binary(binary) =>
                        AddExprResult::Expression(Expression::Binary(Box::new(binary))),
                    AddExprResult::Full => AddExprResult::Full,
                    AddExprResult::Error(err) => AddExprResult::Error(err),
                    _ => AddExprResult::Error("Wrong type".to_string()),
                }
            }
            Expression::Unary(current) => {
                match current.add_expr(expr) {
                    AddExprResult::Unary(unary) =>
                        AddExprResult::Expression(Expression::Unary(Box::new(unary))),
                    AddExprResult::Full => AddExprResult::Full,
                    AddExprResult::Error(err) => AddExprResult::Error(err),
                    _ => AddExprResult::Error("Wrong type".to_string()),
                }
            }
            Expression::None => { AddExprResult::Expression(expr) }
            _ => AddExprResult::Expression(expr),
        }
    }
}

impl AddExpression for Binary {
    fn add_expr(&mut self, expr: Expression) -> AddExprResult {
        match self.clone() {
            Self::EqualEqual(e1, _) => AddExprResult::Binary(Self::EqualEqual(e1, expr.clone())),
            Self::BangEqual(e1, _) => AddExprResult::Binary(Self::BangEqual(e1, expr.clone())),
            Self::Greater(e1, _) => AddExprResult::Binary(Self::Greater(e1, expr.clone())),
            Self::GreaterEqual(e1, _) =>
                AddExprResult::Binary(Self::GreaterEqual(e1, expr.clone())),
            Self::Less(e1, _) => AddExprResult::Binary(Self::Less(e1, expr.clone())),
            Self::LessEqual(e1, _) => AddExprResult::Binary(Self::LessEqual(e1, expr.clone())),
            Self::Plus(e1, _) => AddExprResult::Binary(Self::Plus(e1, expr.clone())),
            Self::Minus(e1, _) => AddExprResult::Binary(Self::Minus(e1, expr.clone())),
            Self::Star(e1, _) => AddExprResult::Binary(Self::Star(e1, expr.clone())),
            Self::Slash(e1, _) => AddExprResult::Binary(Self::Slash(e1, expr.clone())),
        }
    }
}

impl AddExpression for Unary {
    fn add_expr(&mut self, expr: Expression) -> AddExprResult {
        match self {
            Unary::Bang(current) =>
                match current.add_expr(expr) {
                    AddExprResult::Expression(new_expr) => {
                        AddExprResult::Unary(Unary::Bang(new_expr))
                    }
                    AddExprResult::Full => AddExprResult::Full,
                    AddExprResult::Error(err) => AddExprResult::Error(err),
                    _ => AddExprResult::Error("Wrong type".to_string()),
                }
            Unary::Minus(current) => current.add_expr(expr),
        }
    }
}

pub enum AddExprResult {
    Expression(Expression),
    Binary(Binary),
    Unary(Unary),
    Full,
    Error(String),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Grouping(expr) => { format!("(group {})", expr.to_string()) }
            Expression::Binary(binary) => {
                let binary = binary.as_ref();
                match binary {
                    Binary::Plus(expr1, expr2) =>
                        format!("(+ {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::Minus(expr1, expr2) =>
                        format!("(- {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::Star(expr1, expr2) =>
                        format!("(* {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::EqualEqual(expr1, expr2) =>
                        format!("(== {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::BangEqual(expr1, expr2) =>
                        format!("(!= {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::Greater(expr1, expr2) =>
                        format!("(> {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::GreaterEqual(expr1, expr2) =>
                        format!("(>= {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::Less(expr1, expr2) =>
                        format!("(< {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::LessEqual(expr1, expr2) =>
                        format!("(<= {} {})", expr1.to_string(), expr2.to_string()),
                    Binary::Slash(expr1, expr2) =>
                        format!("(/ {} {})", expr1.to_string(), expr2.to_string()),
                }
            }
            Expression::Unary(_) => todo!(),
            Expression::Number(number) => format!("{number}"),
            Expression::String(string) => format!("{string}"),
            Expression::True => format!("true"),
            Expression::False => format!("true"),
            Expression::Nil => format!("nil"),
            Expression::None => format!(""),
        }
    }
}
