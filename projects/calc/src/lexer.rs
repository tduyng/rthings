use std::str::CharIndices;

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    chars: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.char_indices();
        Self {
            input,
            pos: 0,
            chars,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\t' | '\n' => self.advance(),
                '#' => self.skip_comment(),
                '0'..='9' => tokens.push(self.lex_number()),
                '+' => tokens.push(self.lex_single_char(TokenKind::Plus)),
                '-' => tokens.push(self.lex_single_char(TokenKind::Minus)),
                '*' => tokens.push(self.lex_single_char(TokenKind::Star)),
                '/' => tokens.push(self.lex_single_char(TokenKind::Slash)),
                '^' => tokens.push(self.lex_single_char(TokenKind::Caret)),
                '(' => tokens.push(self.lex_single_char(TokenKind::LParen)),
                ')' => tokens.push(self.lex_single_char(TokenKind::RParen)),
                _ => self.advance(),
            }
        }
        tokens
    }
    fn advance(&mut self) {
        self.chars.next();
        self.pos += 1;
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|(_, c)| c)
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        let mut has_dot = false;
        let mut has_exponent = false;

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' | '_' => {
                    self.advance();
                }
                '.' if !has_dot => {
                    has_dot = true;
                    self.advance();
                }
                'e' | 'E' if !has_exponent => {
                    has_exponent = true;
                    self.advance();
                    if let Some(next_char) = self.peek() {
                        if next_char == '+' || next_char == '-' {
                            self.advance();
                        }
                    }
                }
                _ => break,
            }
        }

        let end = self.pos;
        let number_str: String = self.input[start..end]
            .chars()
            .filter(|&c| c != '_')
            .collect();
        let number = number_str.parse().unwrap_or(0.0); // Handle parsing errors appropriately in real code
        Token {
            start,
            end,
            kind: TokenKind::Number(number),
        }
    }

    fn lex_single_char(&mut self, kind: TokenKind) -> Token {
        let start = self.pos;
        self.advance();
        Token {
            start,
            end: self.pos,
            kind,
        }
    }
}
