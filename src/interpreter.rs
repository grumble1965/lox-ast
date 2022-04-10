use crate::error::LoxError;
use crate::expr::*;
use crate::object::Object;
use crate::token::*;
use crate::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    pub fn interpret(&self, expr: &Expr) -> Result<(), ()> {
        match self.evaluate(expr) {
            Ok(value) => {
                println!("{}", value);
                Ok(())
            }
            Err(err) => {
                err.report("");
                Err(())
            }
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
        let op = expr.operator.token_type();

        // break out the cases where left and right are the same flavor
        let result: Object = match (left, right) {
            (Object::Num(left), Object::Num(right)) => match op {
                TokenType::Plus => Object::Num(left + right),
                TokenType::Minus => Object::Num(left - right),
                TokenType::Star => Object::Num(left * right),
                TokenType::Slash => Object::Num(left / right),
                TokenType::Greater => Object::Bool(left > right),
                TokenType::GreaterEqual => Object::Bool(left >= right),
                TokenType::Less => Object::Bool(left < right),
                TokenType::LessEqual => Object::Bool(left <= right),
                TokenType::EqualEqual => Object::Bool(left == right),
                TokenType::BangEqual => Object::Bool(left != right),
                _ => Object::ArithmeticError,
            },
            (Object::Str(left), Object::Str(right)) => match op {
                TokenType::Plus => Object::Str(format!("{}{}", left, right)),
                TokenType::Greater => Object::Bool(left > right),
                TokenType::GreaterEqual => Object::Bool(left >= right),
                TokenType::Less => Object::Bool(left < right),
                TokenType::LessEqual => Object::Bool(left <= right),
                TokenType::EqualEqual => Object::Bool(left == right),
                TokenType::BangEqual => Object::Bool(left != right),
                _ => Object::ArithmeticError,
            },
            (Object::Bool(left), Object::Bool(right)) => match op {
                TokenType::EqualEqual => Object::Bool(left == right),
                TokenType::BangEqual => Object::Bool(left != right),
                _ => Object::ArithmeticError,
            },
            (Object::Nil, Object::Nil) => match op {
                TokenType::EqualEqual => Object::Bool(true),
                TokenType::BangEqual => Object::Bool(false),
                _ => Object::ArithmeticError,
            },
            (Object::Nil, _) => match op {
                TokenType::EqualEqual => Object::Bool(false),
                TokenType::BangEqual => Object::Bool(true),
                _ => Object::ArithmeticError,
            },
            (_, _) => match op {
                TokenType::EqualEqual => Object::Bool(false),
                TokenType::BangEqual => Object::Bool(true),
                _ => Object::ArithmeticError,
            },
        };

        match result {
            Object::ArithmeticError => Err(LoxError::runtime_error(
                &expr.operator,
                "Invalid Arithmetic Expression",
            )),
            a => Ok(a),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        match &expr.value {
            Some(value) => Ok(value.clone()),
            _ => Err(LoxError::runtime_error(
                &Token {
                    ttype: TokenType::Eof,
                    lexeme: "".to_string(),
                    literal: None,
                    line: 0,
                },
                "invalid literal value",
            )),
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        let result: Result<Object, LoxError> = match expr.operator.token_type() {
            TokenType::Minus => Ok(-right),
            TokenType::Bang => Ok(!right),
            _ => Err(LoxError::runtime_error(
                &expr.operator,
                "unhandled unary operator",
            )),
        };

        if matches!(result, Ok(Object::ArithmeticError)) {
            Err(LoxError::runtime_error(
                &expr.operator,
                "Invalid Arithmetic Expression",
            ))
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invoke_unary(operator: Token, right: Object) -> Result<Object, LoxError> {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator,
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(right) })),
        };
        terp.visit_unary_expr(&unary_expr)
    }

    #[test]
    fn unary_minus() {
        let op1 = Token::new(TokenType::Minus, "-".to_string(), None, 10);
        let result = invoke_unary(op1, Object::Num(123.0));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(-123.0)));

        let op2 = Token::new(TokenType::Minus, "-".to_string(), None, 10);
        let err_result = invoke_unary(op2, Object::Nil);
        assert!(err_result.is_err());
    }

    #[test]
    fn unary_not() {
        let op = Token::new(TokenType::Bang, "!".to_string(), None, 10);
        let result = invoke_unary(op, Object::Bool(true));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn unary_rejects_unsupported_ops() {
        let op = Token::new(TokenType::Star, "*".to_string(), None, 10);
        let err_result = invoke_unary(op, Object::Bool(true));
        assert!(err_result.is_err());
    }

    fn invoke_binary(left: Object, operator: Token, right: Object) -> Result<Object, LoxError> {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Literal(LiteralExpr { value: Some(left) })),
            operator,
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(right) })),
        };
        terp.visit_binary_expr(&binary_expr)
    }

    #[test]
    fn binary_minus() {
        let op1 = Token::new(TokenType::Minus, "-".to_string(), None, 10);
        let result = invoke_binary(Object::Num(123.0), op1, Object::Num(23.0));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(100.0)));

        let op2 = Token::new(TokenType::Minus, "-".to_string(), None, 10);
        let err_result = invoke_binary(Object::Num(100.0), op2, Object::Nil);
        assert!(err_result.is_err());
    }

    #[test]
    fn binary_divide() {
        let op1 = Token::new(TokenType::Slash, "/".to_string(), None, 10);
        let result = invoke_binary(Object::Num(500.0), op1, Object::Num(25.0));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(20.0)));

        let op2 = Token::new(TokenType::Slash, "/".to_string(), None, 10);
        let err_result = invoke_binary(Object::Num(500.0), op2, Object::Str("".to_string()));
        assert!(err_result.is_err());
    }

    #[test]
    fn binary_times() {
        let op1 = Token::new(TokenType::Star, "*".to_string(), None, 10);
        let result = invoke_binary(Object::Num(10.0), op1, Object::Num(25.0));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(250.0)));

        let op2 = Token::new(TokenType::Star, "*".to_string(), None, 10);
        let err_result = invoke_binary(Object::Num(500.0), op2, Object::Bool(false));
        assert!(err_result.is_err());
    }

    #[test]
    fn binary_numeric_plus() {
        let op1 = Token::new(TokenType::Plus, "+".to_string(), None, 10);
        let result = invoke_binary(Object::Num(10.0), op1, Object::Num(25.0));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Num(35.0)));

        let op2 = Token::new(TokenType::Plus, "+".to_string(), None, 10);
        let err_result = invoke_binary(Object::Num(500.0), op2, Object::Bool(false));
        assert!(err_result.is_err());
    }

    #[test]
    fn binary_string_concat() {
        let op1 = Token::new(TokenType::Plus, "+".to_string(), None, 10);
        let result = invoke_binary(
            Object::Str("abc".to_string()),
            op1,
            Object::Str("def".to_string()),
        );
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Str("abcdef".to_string())));
    }

    #[test]
    fn binary_greater_mixed_is_error() {
        let op1 = Token::new(TokenType::Greater, ">".to_string(), None, 10);
        let result = invoke_binary(Object::Num(4.0), op1, Object::Str("foo".to_string()));
        assert!(result.is_err());
    }

    fn run_comparison_test(tok: &Token, cmps: Vec<bool>) {
        let nums = vec![-10.0, 0.0, 10.0];
        let comp_num = 0.0;

        for (c, num) in cmps.iter().zip(nums) {
            let result = invoke_binary(Object::Num(num), tok.dup(), Object::Num(comp_num));
            assert!(result.is_ok());
            assert_eq!(
                result.ok(),
                Some(Object::Bool(*c)),
                "Testing {} {} {}",
                num,
                tok.lexeme,
                comp_num
            );
        }
    }

    #[test]
    fn binary_greater_than() {
        run_comparison_test(
            &Token::new(TokenType::Greater, ">".to_string(), None, 10),
            vec![false, false, true],
        )
    }

    #[test]
    fn binary_greater_than_or_equal() {
        run_comparison_test(
            &Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 10),
            vec![false, true, true],
        )
    }

    #[test]
    fn binary_less_than() {
        run_comparison_test(
            &Token::new(TokenType::Less, "<".to_string(), None, 10),
            vec![true, false, false],
        )
    }

    #[test]
    fn binary_less_than_or_equal() {
        run_comparison_test(
            &Token::new(TokenType::LessEqual, "<=".to_string(), None, 10),
            vec![true, true, false],
        )
    }

    #[test]
    fn binary_equal_num() {
        run_comparison_test(
            &Token::new(TokenType::EqualEqual, "==".to_string(), None, 10),
            vec![false, true, false],
        )
    }

    #[test]
    fn binary_not_equal_num() {
        run_comparison_test(
            &Token::new(TokenType::BangEqual, "!=".to_string(), None, 10),
            vec![true, false, true],
        )
    }

    #[test]
    fn binary_equal_str() {
        let op1 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result = invoke_binary(
            Object::Str("testing".to_string()),
            op1,
            Object::Str("testing".to_string()),
        );
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        let op2 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result2 = invoke_binary(
            Object::Str("foo".to_string()),
            op2,
            Object::Str("bar".to_string()),
        );
        assert!(result2.is_ok());
        assert_eq!(result2.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn binary_equal_bool() {
        let op1 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result = invoke_binary(Object::Bool(true), op1, Object::Bool(true));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));

        let op2 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result2 = invoke_binary(Object::Bool(true), op2, Object::Bool(false));
        assert!(result2.is_ok());
        assert_eq!(result2.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn binary_equal_nil() {
        let op1 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result = invoke_binary(Object::Nil, op1, Object::Nil);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn binary_equal_nil_mixed() {
        let op1 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result = invoke_binary(Object::Nil, op1, Object::Str("three".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn binary_equal_nonnil_mixed() {
        let op1 = Token::new(TokenType::EqualEqual, "==".to_string(), None, 10);
        let result = invoke_binary(Object::Num(3.0), op1, Object::Str("three".to_string()));
        // this should generate a runtime error
        // assert!(result.is_err());
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(false)));
    }

    #[test]
    fn binary_rejects_unsupported_ops() {
        let op2 = Token::new(TokenType::If, "if".to_string(), None, 10);
        let err_result = invoke_binary(Object::Num(500.0), op2, Object::Bool(false));
        assert!(err_result.is_err());
    }
}
