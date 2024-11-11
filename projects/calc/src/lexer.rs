use std::{
    fs::File,
    io::{BufReader, Lines},
    vec,
};

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    pos: usize,
    line: Option<String>,
    lines: Lines<BufReader<File>>,
}

impl Lexer {
    pub fn new(lines: Lines<BufReader<File>>) -> Self {
        let mut lexer = Lexer {
            pos: 0,
            line: None,
            lines,
        };
        lexer.advance_line();
        lexer
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while self.line.is_some() {
            if let Some(c) = self.char() {
                match c {
                    ' ' | '\t' => self.advance(),
                    '#' => self.skip_comment(),
                    '0'..='9' => tokens.push(self.lex_number()),
                    '+' => tokens.push(self.single_char_token(TokenKind::Plus)),
                    '-' => tokens.push(self.single_char_token(TokenKind::Minus)),
                    '*' => tokens.push(self.single_char_token(TokenKind::Star)),
                    '/' => tokens.push(self.single_char_token(TokenKind::Slash)),
                    '^' => tokens.push(self.single_char_token(TokenKind::Caret)),
                    '(' => tokens.push(self.single_char_token(TokenKind::LParen)),
                    ')' => tokens.push(self.single_char_token(TokenKind::RParen)),
                    _ => self.advance(),
                }
            } else {
                self.advance_line();
            }
        }
        dbg!(&tokens);
        tokens
    }

    fn advance_line(&mut self) {
        self.pos = 0;
        self.line = self.lines.next().and_then(|res| res.ok());
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn char(&self) -> Option<char> {
        self.line.as_ref()?.chars().nth(self.pos)
    }

    fn skip_comment(&mut self) {
        while self.char().is_some() {
            self.advance();
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        let mut has_dot = false;
        let mut has_exponent = false;

        while let Some(c) = self.char() {
            match c {
                '0'..='9' | '_' => self.advance(),
                '.' if !has_dot => {
                    has_dot = true;
                    self.advance();
                }
                'e' | 'E' if !has_exponent => {
                    has_exponent = true;
                    self.advance();
                    if let Some(next_char) = self.char() {
                        if next_char == '+' || next_char == '-' {
                            self.advance();
                        }
                    }
                }
                _ => break,
            }
        }

        let end = self.pos;
        let number_str: String = self.line.as_ref().unwrap()[start..end]
            .chars()
            .filter(|&c| c != '_')
            .collect();
        let number = number_str.parse().unwrap();
        Token {
            pos: start,
            kind: TokenKind::Number(number),
        }
    }

    fn single_char_token(&mut self, kind: TokenKind) -> Token {
        let token = Token::new(self.pos, kind);
        self.advance();
        token
    }
}
