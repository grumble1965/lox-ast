use crate::error::LoxError;
use crate::expr::*;
use crate::object::Object;
use crate::token::*;
use crate::token_type::TokenType;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            _ => None,
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.term()?;

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            }));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(LoxError::error(self.previous().line, "Expect expression."))
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for &ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn error(token: Token, message: &str) -> Result<Token, LoxError> {
        Err(LoxError::parse_error(&token, message))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }

            if matches!(
                self.peek().token_type(),
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }

            self.advance();
        }
    }

    fn consume(&mut self, ttype: TokenType, err_msg: &str) -> Result<Token, LoxError> {
        if self.check(ttype) {
            Ok(self.advance().dup())
        } else {
            Parser::error(self.peek().dup(), err_msg)
        }
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().is(ttype)
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is(TokenType::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap();
    }
}
