use crate::error::LoxError;
use crate::expr::*;
use crate::object::Object;
use crate::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn is_truthy(&self, obj: &Object) -> bool {
        !matches!(obj, Object::False | Object::Nil)
    }

    fn is_numeric(&self, obj: &Object) -> bool {
        matches!(obj, Object::Num(_n))
    }

    fn is_string(&self, obj: &Object) -> bool {
        matches!(obj, Object::Str(_s))
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match left {
                Object::Num(n_left) => match right {
                    Object::Num(n_right) => Ok(Object::Num(n_left - n_right)),
                    _ => Err(LoxError::error(0, "right expression not numeric")),
                },
                _ => Err(LoxError::error(0, "left expression not numeric")),
            },
            TokenType::Plus => {
                if self.is_numeric(&left) && self.is_numeric(&right) {
                    match left {
                        Object::Num(l_num) => match right {
                            Object::Num(r_num) => Ok(Object::Num(l_num + r_num)),
                            _ => Ok(Object::Nil),
                        },
                        _ => Ok(Object::Nil),
                    }
                } else if self.is_string(&left) && self.is_string(&right) {
                    match left {
                        Object::Str(l_str) => match right {
                            Object::Str(r_str) => {
                                let concat = format!("{}{}", l_str, r_str);
                                Ok(Object::Str(concat))
                            }
                            _ => Ok(Object::Nil),
                        },
                        _ => Ok(Object::Nil),
                    }
                } else {
                    Err(LoxError::error(
                        0,
                        "left and right expression must match and be numbers or strings",
                    ))
                }
            }
            TokenType::Slash => match left {
                Object::Num(n_left) => match right {
                    Object::Num(n_right) => Ok(Object::Num(n_left / n_right)),
                    _ => Err(LoxError::error(0, "right expression not numeric")),
                },
                _ => Err(LoxError::error(0, "left expression not numeric")),
            },
            TokenType::Star => match left {
                Object::Num(n_left) => match right {
                    Object::Num(n_right) => Ok(Object::Num(n_left * n_right)),
                    _ => Err(LoxError::error(0, "right expression not numeric")),
                },
                _ => Err(LoxError::error(0, "left expression not numeric")),
            },
            _ => Err(LoxError::error(0, "unhandled binary opertor")),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        match &expr.value {
            Some(value) => Ok(value.clone()),
            _ => Err(LoxError::error(0, "invalid literal value")),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match right {
                Object::Num(n) => Ok(Object::Num(-n)),
                _ => Err(LoxError::error(0, "unary minus only applies to numbers")),
            },
            TokenType::Bang => {
                if !self.is_truthy(&right) {
                    Ok(Object::True)
                } else {
                    Ok(Object::False)
                }
            }
            _ => return Err(LoxError::error(0, "unhandled unary operator")),
        }
    }
}
