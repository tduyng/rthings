#[derive(Debug)]
pub enum TokenKind {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(start: usize, end: usize, kind: TokenKind) -> Self {
        Token { start, end, kind }
    }
}
