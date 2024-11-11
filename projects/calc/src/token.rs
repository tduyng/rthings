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
    pub pos: usize,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(pos: usize, kind: TokenKind) -> Self {
        Token { pos, kind }
    }
}
