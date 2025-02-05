use super::{AddExpr, Expression, IsPartial};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryType {
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binary {
    binary_type: BinaryType,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Binary {
    pub(crate) fn new(binary_type: BinaryType, left: Expression, right: Expression) -> Self {
        Self {
            binary_type,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub(crate) fn new_empty_expr(binary_type: BinaryType) -> Expression {
        Expression::Binary(Self::new(binary_type, Expression::None, Expression::None))
    }

    fn new_expr(binary_type: BinaryType, left: Expression, right: Expression) -> Expression {
        Expression::Binary(Self::new(binary_type, left, right))
    }
}

impl IsPartial for Binary {
    fn is_partial(&self) -> bool {
        self.left.is_partial() || self.right.is_partial()
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self.binary_type {
            BinaryType::EqualEqual => "==",
            BinaryType::BangEqual => "!=",
            BinaryType::Greater => ">",
            BinaryType::GreaterEqual => ">=",
            BinaryType::Less => "<",
            BinaryType::LessEqual => "<=",
            BinaryType::Plus => "+",
            BinaryType::Minus => "-",
            BinaryType::Star => "*",
            BinaryType::Slash => "/",
        };

        write!(f, "({} {} {})", ch, self.left, self.right)
    }
}

impl AddExpr for Binary {
    fn add_expr(&self, expr: Expression) -> Result<Expression, String> {
        match self.binary_type {
            BinaryType::Plus | BinaryType::Minus
                if expr.is_binary()
                    && self.is_full()
                    && matches!(
                        expr.get_binary()?.binary_type,
                        BinaryType::Slash | BinaryType::Star
                    ) =>
            {
                let b = expr.get_binary()?;
                Ok(Self::new_expr(
                    self.binary_type.clone(),
                    self.left.as_ref().clone(),
                    Self::new_expr(
                        b.binary_type.clone(),
                        self.right.as_ref().clone(),
                        b.left.as_ref().clone(),
                    ),
                ))
            }
            _ if expr.is_binary() && expr.is_partial() => {
                expr.add_expr(Expression::Binary(self.clone()))
            }
            _ => {
                if self.left.is_none() {
                    Ok(Self::new_expr(
                        self.binary_type.clone(),
                        expr,
                        self.right.as_ref().clone(),
                    ))
                } else if self.left.is_partial() {
                    Ok(Self::new_expr(
                        self.binary_type.clone(),
                        self.left.add_expr(expr)?,
                        self.right.as_ref().clone(),
                    ))
                } else if self.right.is_none() {
                    Ok(Self::new_expr(
                        self.binary_type.clone(),
                        self.left.as_ref().clone(),
                        expr,
                    ))
                } else if self.right.is_partial() {
                    Ok(Self::new_expr(
                        self.binary_type.clone(),
                        self.left.as_ref().clone(),
                        self.right.add_expr(expr)?,
                    ))
                } else {
                    panic!("Unkown Binary operation")
                }
            }
        }
    }
}
