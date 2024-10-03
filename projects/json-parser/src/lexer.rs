use std::str::Chars;

use crate::error::LexError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    EOF,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            input: source.chars(),
            current: None,
            position: 0,
        };
        lexer.bump(); // Initialize first character
        lexer
    }

    fn bump(&mut self) {
        self.current = self.input.next();
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        while let Some(c) = self.current {
            match c {
                '{' => {
                    self.bump();
                    return Ok(Token::LBrace);
                }
                '}' => {
                    self.bump();
                    return Ok(Token::RBrace);
                }
                '[' => {
                    self.bump();
                    return Ok(Token::LBracket);
                }
                ']' => {
                    self.bump();
                    return Ok(Token::RBracket);
                }
                ':' => {
                    self.bump();
                    return Ok(Token::Colon);
                }
                ',' => {
                    self.bump();
                    return Ok(Token::Comma);
                }
                'n' => return self.lex_null(),
                't' => return self.lex_true(),
                'f' => return self.lex_false(),
                '"' => return self.lex_string(),
                '0'..='9' | '-' => return self.lex_number(),
                _ if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                _ => return Err(LexError::UnexpectedChar(c, self.position)),
            }
        }
        Ok(Token::EOF) // End of input
    }

    fn lex_null(&mut self) -> Result<Token, LexError> {
        self.expect_keyword("null")?;
        Ok(Token::Null)
    }

    fn lex_true(&mut self) -> Result<Token, LexError> {
        self.expect_keyword("true")?;
        Ok(Token::Boolean(true))
    }

    fn lex_false(&mut self) -> Result<Token, LexError> {
        self.expect_keyword("false")?;
        Ok(Token::Boolean(false))
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), LexError> {
        for expected in keyword.chars() {
            if Some(expected) != self.current {
                return Err(LexError::UnexpectedChar(
                    self.current.unwrap_or('\0'),
                    self.position,
                ));
            }
            self.bump();
        }
        Ok(())
    }

    fn lex_string(&mut self) -> Result<Token, LexError> {
        self.bump(); // Skip opening quote
        let mut value = String::new();
        while let Some(c) = self.current {
            match c {
                '"' => {
                    self.bump(); // Skip closing quote
                    return Ok(Token::String(value));
                }
                '\\' => {
                    let escaped = self.read_escape_sequence()?;
                    value.push(escaped);
                }
                _ => value.push(c),
            }
            self.bump();
        }
        Err(LexError::UnterminatedString(self.position)) // Error for unterminated string
    }

    fn read_escape_sequence(&mut self) -> Result<char, LexError> {
        if let Some(c) = self.current {
            self.bump(); // Consume the escape character
            match c {
                '"' => Ok('"'),
                '\\' => Ok('\\'),
                '/' => Ok('/'),
                'b' => Ok('\u{0008}'),
                'f' => Ok('\u{000C}'),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                'u' => {
                    let mut hex = String::new();
                    for _ in 0..4 {
                        if let Some(h) = self.current {
                            if h.is_digit(16) {
                                hex.push(h);
                                self.bump();
                            } else {
                                return Err(LexError::UnexpectedChar(h, self.position));
                            }
                        } else {
                            return Err(LexError::UnterminatedString(self.position));
                        }
                    }
                    let code_point = u32::from_str_radix(&hex, 16)
                        .map_err(|_| LexError::UnexpectedChar('\0', self.position))?;
                    Ok(std::char::from_u32(code_point)
                        .ok_or_else(|| LexError::UnexpectedChar('\0', self.position))?)
                }
                _ => Err(LexError::UnexpectedChar(c, self.position)),
            }
        } else {
            Err(LexError::UnterminatedString(self.position))
        }
    }

    fn lex_number(&mut self) -> Result<Token, LexError> {
        let mut number_str = String::new();
        while let Some(c) = self.current {
            if c.is_digit(10) || c == '.' || c == '-' {
                number_str.push(c);
                self.bump();
            } else {
                break;
            }
        }

        match number_str.parse::<f64>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(LexError::InvalidNumber(number_str, self.position)),
        }
    }
}
