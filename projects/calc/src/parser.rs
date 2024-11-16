use crate::{
    expr::{Binary, Constant, Node, Unary, Variable},
    token::{Token, TokenType},
    types::Value,
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Option<Box<dyn Node>>> {
        let mut nodes = vec![];
        while !self.at_end() {
            nodes.push(self.expression());
        }
        nodes
    }

    fn expression(&mut self) -> Option<Box<dyn Node>> {
        match self.peek()?.t {
            TokenType::Ident(_) => self.parse_ident(),
            _ => self.parse_term(),
        }
    }

    fn parse_ident(&mut self) -> Option<Box<dyn Node>> {
        let identifier = self.peek()?.t.clone();
        self.advance();

        if self.check(TokenType::Equal) {
            let name = if let TokenType::Ident(ident) = identifier {
                ident
            } else {
                panic!("Unexpected state in parse_ident");
            };
            self.advance();
            let rhs = self.parse_primary();
            return Some(Box::new(Variable {
                ident: name,
                value: rhs,
            }));
        }
        Some(Box::new(Constant { t: identifier }))
    }

    fn parse_term(&mut self) -> Option<Box<dyn Node>> {
        let mut lhs = self.parse_factor();
        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.prev()?.t.clone();
            let rhs = self.parse_factor();
            lhs = Some(Box::new(Binary {
                t: op,
                left: lhs,
                right: rhs,
            }))
        }
        lhs
    }

    fn parse_factor(&mut self) -> Option<Box<dyn Node>> {
        let mut lhs = self.parse_unary();
        while self.matches(&[TokenType::Slash, TokenType::Asteriks]) {
            let op = self.prev()?.t.clone();
            let rhs = self.parse_unary();
            lhs = Some(Box::new(Binary {
                t: op,
                left: lhs,
                right: rhs,
            }))
        }
        lhs
    }

    fn parse_unary(&mut self) -> Option<Box<dyn Node>> {
        if self.matches(&[TokenType::Minus]) {
            let rhs = self.parse_unary();
            return Some(Box::new(Unary { right: rhs }));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Box<dyn Node>> {
        match self.peek()?.t {
            TokenType::String(_) | TokenType::Number(_) => {
                let op = self.peek()?.t.clone();
                self.advance();
                Some(Box::new(Constant { t: op }))
            }
            TokenType::BraceLeft => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::BraceRight, "Expected ')'");
                expr
            }
            _ => panic!("Expected expression"),
        }
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, expected: TokenType, error_msg: &str) {
        if self.check(expected.clone()) {
            self.advance();
        } else {
            let actual = self.peek().unwrap().t.clone();
            panic!("{} - Expected {:?}, got {:?}", error_msg, expected, actual);
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.peek().map_or(false, |token| token.t == token_type)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn prev(&self) -> Option<&Token> {
        self.tokens.get(self.pos.saturating_sub(1))
    }
}
