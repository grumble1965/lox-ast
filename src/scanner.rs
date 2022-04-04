use crate::error::LoxError;
use crate::token::Object;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {}
                Err(e) => {
                    e.report("".to_string());
                    had_error = Some(e);
                }
            };
        }

        self.tokens.push(Token::eof(self.current));

        match had_error {
            None => Ok(&self.tokens),
            Some(e) => Err(e),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.next_char_matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.next_char_matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.next_char_matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.next_char_matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            }
            '/' => {
                if self.next_char_matches('/') {
                    // A comment extends to the end of the line
                    while self.peek() != None && self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                self.string()?;
            }
            _ => {
                if Scanner::is_digit(Some(c)) {
                    self.number();
                } else if Scanner::is_alpha(Some(c)) {
                    self.identifier();
                } else {
                    return Err(LoxError::error(
                        self.line,
                        "Unexpected character.".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != None && self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }

        // eat the closing "
        self.advance();

        // Trim the surrounding quotes
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            // Consume the .
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(num)));
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(ttype) = Scanner::keyword(&text) {
            self.add_token(ttype);
        } else {
            self.add_token(TokenType::Identifier)
        }
    }

    fn is_digit(ch: Option<char>) -> bool {
        matches!(ch, Some('0'..='9'))
    }

    fn is_alpha(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
        } else {
            false
        }
    }

    fn is_alphanumeric(ch: Option<char>) -> bool {
        Scanner::is_alpha(ch) || Scanner::is_digit(ch)
    }

    fn advance(&mut self) -> char {
        let result = self.source[self.current];
        self.current += 1;
        result
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line));
    }

    fn next_char_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
