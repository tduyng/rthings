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
        let mut l = Lexer {
            pos: 0,
            line: None,
            lines,
        };
        l.advance();
        l
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let t = vec![];
        loop {
            let cc: char;
            if let Some(c) = self.char() {
                cc = c;
            } else {
                break;
            }
            dbg!(cc);
            self.advance();
        }
        t
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos >= self.line.as_ref().unwrap_or(&String::new()).len() || self.line.is_none() {
            self.line = match self.lines.next() {
                Some(Ok(line)) => Some(line),
                _ => None,
            };
        }
    }

    fn char(&self) -> Option<char> {
        self.line.as_ref()?.chars().nth(self.pos)
    }
}
