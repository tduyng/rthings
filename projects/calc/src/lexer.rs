use std::{
    fs::File,
    io::{BufReader, Lines},
    vec,
};

use crate::token::Token;

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
        let tokens = vec![];
        while self.line.is_some() {
            if let Some(c) = self.char() {
                match c {
                    ' ' | '\t' => self.advance(),
                    '#' => self.skip_comment(),
                    _ => self.advance(),
                }
            } else {
                self.advance_line();
            }
        }
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
}
