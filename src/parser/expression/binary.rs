use super::{AddExpr, AddExprResult, Expression};

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

impl Binary {
    pub fn is_full(&self) -> bool {
        match self {
            Binary::EqualEqual(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::BangEqual(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::Greater(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::GreaterEqual(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::Less(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::LessEqual(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::Plus(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::Minus(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::Star(e1, e2) => !(e1.is_none() || e2.is_none()),
            Binary::Slash(e1, e2) => !(e1.is_none() || e2.is_none()),
        }
    }

    pub fn has_slot(&self) -> bool {
        match self {
            Binary::EqualEqual(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::BangEqual(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::Greater(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::GreaterEqual(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::Less(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::LessEqual(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::Plus(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::Minus(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::Star(e1, e2) => e1.is_none() || e2.is_none(),
            Binary::Slash(e1, e2) => e1.is_none() || e2.is_none(),
        }
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary::Plus(e1, e2) => {
                write!(f, "(+ {} {})", e1.to_string(), e2.to_string())
            }
            Binary::Minus(e1, e2) => {
                write!(f, "(- {} {})", e1.to_string(), e2.to_string())
            }
            Binary::Star(e1, e2) => {
                write!(f, "(* {} {})", e1.to_string(), e2.to_string())
            }
            Binary::EqualEqual(e1, e2) => {
                write!(f, "(== {} {})", e1.to_string(), e2.to_string())
            }
            Binary::BangEqual(e1, e2) => {
                write!(f, "(!= {} {})", e1.to_string(), e2.to_string())
            }
            Binary::Greater(e1, e2) => {
                write!(f, "(> {} {})", e1.to_string(), e2.to_string())
            }
            Binary::GreaterEqual(e1, e2) => {
                write!(f, "(>= {} {})", e1.to_string(), e2.to_string())
            }
            Binary::Less(e1, e2) => {
                write!(f, "(< {} {})", e1.to_string(), e2.to_string())
            }
            Binary::LessEqual(e1, e2) => {
                write!(f, "(<= {} {})", e1.to_string(), e2.to_string())
            }
            Binary::Slash(e1, e2) => {
                write!(f, "(/ {} {})", e1.to_string(), e2.to_string())
            }
        }
    }
}

impl AddExpr for Binary {
    fn add_expr(&self, expr: Expression) -> AddExprResult {
        match self {
            Binary::EqualEqual(e1, e2) if e1.is_binary() && e1.has_slot() => {
                match e1.add_expr(expr) {
                    AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                        Binary::EqualEqual(ne, e2.clone()),
                    ))),
                    r => r,
                }
            }
            Binary::EqualEqual(e1, e2) if e1.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::EqualEqual(expr, e2.clone())),
            )),
            Binary::EqualEqual(e1, e2) if e2.is_binary() && e2.has_slot() => {
                match e2.add_expr(expr) {
                    AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                        Binary::EqualEqual(e1.clone(), ne),
                    ))),
                    r => r,
                }
            }
            Binary::EqualEqual(e1, e2) if e2.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::EqualEqual(e1.clone(), expr)),
            )),
            Binary::BangEqual(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr)
            {
                AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                    Binary::BangEqual(ne, e2.clone()),
                ))),
                r => r,
            },
            Binary::BangEqual(e1, e2) if e1.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::BangEqual(expr, e2.clone())),
            )),
            Binary::BangEqual(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr)
            {
                AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                    Binary::BangEqual(e1.clone(), ne),
                ))),
                r => r,
            },
            Binary::BangEqual(e1, e2) if e2.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::BangEqual(e1.clone(), expr)),
            )),
            Binary::GreaterEqual(e1, e2) if e1.is_binary() && e1.has_slot() => {
                match e1.add_expr(expr) {
                    AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                        Binary::GreaterEqual(ne, e2.clone()),
                    ))),
                    r => r,
                }
            }
            Binary::GreaterEqual(e1, e2) if e1.is_none() => AddExprResult::Done(
                Expression::Binary(Box::new(Binary::GreaterEqual(expr, e2.clone()))),
            ),
            Binary::GreaterEqual(e1, e2) if e2.is_binary() && e2.has_slot() => {
                match e2.add_expr(expr) {
                    AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                        Binary::GreaterEqual(e1.clone(), ne),
                    ))),
                    r => r,
                }
            }
            Binary::GreaterEqual(e1, e2) if e2.is_none() => AddExprResult::Done(
                Expression::Binary(Box::new(Binary::GreaterEqual(e1.clone(), expr))),
            ),
            Binary::Greater(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr) {
                AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                    Binary::Greater(ne, e2.clone()),
                ))),
                r => r,
            },
            Binary::Greater(e1, e2) if e1.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::Greater(expr, e2.clone())),
            )),
            Binary::Greater(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr) {
                AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                    Binary::Greater(e1.clone(), ne),
                ))),
                r => r,
            },
            Binary::Greater(e1, e2) if e2.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::Greater(e1.clone(), expr)),
            )),
            Binary::LessEqual(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr)
            {
                AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                    Binary::LessEqual(ne, e2.clone()),
                ))),
                r => r,
            },
            Binary::LessEqual(e1, e2) if e1.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::LessEqual(expr, e2.clone())),
            )),
            Binary::LessEqual(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr)
            {
                AddExprResult::Done(ne) => AddExprResult::Done(Expression::Binary(Box::new(
                    Binary::LessEqual(e1.clone(), ne),
                ))),
                r => r,
            },
            Binary::LessEqual(e1, e2) if e2.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::LessEqual(e1.clone(), expr)),
            )),
            Binary::Less(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Less(ne, e2.clone()))))
                }
                r => r,
            },
            Binary::Less(e1, e2) if e1.is_none() => {
                AddExprResult::Done(Expression::Binary(Box::new(Binary::Less(expr, e2.clone()))))
            }
            Binary::Less(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Less(e1.clone(), ne))))
                }
                r => r,
            },
            Binary::Less(e1, e2) if e2.is_none() => {
                AddExprResult::Done(Expression::Binary(Box::new(Binary::Less(e1.clone(), expr))))
            }
            Binary::Plus(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Plus(ne, e2.clone()))))
                }
                r => r,
            },
            Binary::Plus(e1, e2) if e1.is_none() => {
                AddExprResult::Done(Expression::Binary(Box::new(Binary::Plus(expr, e2.clone()))))
            }
            Binary::Plus(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Plus(e1.clone(), ne))))
                }
                r => r,
            },
            Binary::Plus(e1, e2) if e2.is_none() => {
                AddExprResult::Done(Expression::Binary(Box::new(Binary::Plus(e1.clone(), expr))))
            }

            Binary::Minus(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Minus(ne, e2.clone()))))
                }
                r => r,
            },
            Binary::Minus(e1, e2) if e1.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::Minus(expr, e2.clone())),
            )),
            Binary::Minus(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Minus(e1.clone(), ne))))
                }
                r => r,
            },
            Binary::Minus(e1, e2) if e2.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::Minus(e1.clone(), expr)),
            )),
            Binary::Star(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Star(ne, e2.clone()))))
                }
                r => r,
            },
            Binary::Star(e1, e2) if e1.is_none() => {
                AddExprResult::Done(Expression::Binary(Box::new(Binary::Star(expr, e2.clone()))))
            }
            Binary::Star(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Star(e1.clone(), ne))))
                }
                r => r,
            },
            Binary::Star(e1, e2) if e2.is_none() => {
                AddExprResult::Done(Expression::Binary(Box::new(Binary::Star(e1.clone(), expr))))
            }
            Binary::Slash(e1, e2) if e1.is_binary() && e1.has_slot() => match e1.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Slash(ne, e2.clone()))))
                }
                r => r,
            },
            Binary::Slash(e1, e2) if e1.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::Slash(expr, e2.clone())),
            )),
            Binary::Slash(e1, e2) if e2.is_binary() && e2.has_slot() => match e2.add_expr(expr) {
                AddExprResult::Done(ne) => {
                    AddExprResult::Done(Expression::Binary(Box::new(Binary::Slash(e1.clone(), ne))))
                }
                r => r,
            },
            Binary::Slash(e1, e2) if e2.is_none() => AddExprResult::Done(Expression::Binary(
                Box::new(Binary::Slash(e1.clone(), expr)),
            )),
            Binary::Plus(e1, e2) if matches!(expr, Expression::Binary(_)) => {
                if let Expression::Binary(ref b) = expr {
                    if let Binary::Star(ne1, _) = b.as_ref() {
                        return AddExprResult::Done(Expression::Binary(Box::new(Binary::Plus(
                            e1.clone(),
                            Expression::Binary(Box::new(Binary::Star(e2.clone(), ne1.clone()))),
                        ))));
                    }
                    if let Binary::Slash(ne1, _) = b.as_ref() {
                        return AddExprResult::Done(Expression::Binary(Box::new(Binary::Plus(
                            e1.clone(),
                            Expression::Binary(Box::new(Binary::Slash(e2.clone(), ne1.clone()))),
                        ))));
                    }

                    return expr.add_expr(Expression::Binary(Box::new(self.clone())));
                }

                expr.add_expr(Expression::Binary(Box::new(self.clone())))
            }
            Binary::Minus(e1, e2) if matches!(expr, Expression::Binary(_)) => {
                if let Expression::Binary(ref b) = expr {
                    if let Binary::Star(ne1, _) = b.as_ref() {
                        return AddExprResult::Done(Expression::Binary(Box::new(Binary::Minus(
                            e1.clone(),
                            Expression::Binary(Box::new(Binary::Star(e2.clone(), ne1.clone()))),
                        ))));
                    }
                    if let Binary::Slash(ne1, _) = b.as_ref() {
                        return AddExprResult::Done(Expression::Binary(Box::new(Binary::Minus(
                            e1.clone(),
                            Expression::Binary(Box::new(Binary::Slash(e2.clone(), ne1.clone()))),
                        ))));
                    }

                    return expr.add_expr(Expression::Binary(Box::new(self.clone())));
                }

                expr.add_expr(Expression::Binary(Box::new(self.clone())))
            }
            _ if matches!(expr, Expression::Binary(_)) => {
                expr.add_expr(Expression::Binary(Box::new(self.clone())))
            }
            _ => AddExprResult::Full,
        }
    }
}
