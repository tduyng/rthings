use crate::{
    error::LexError,
    lexer::{Lexer, Token},
};

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(Vec<(String, Box<JsonValue>)>),
    Array(Vec<Box<JsonValue>>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.bump();
        parser
    }

    fn bump(&mut self) {
        self.current_token = self.lexer.next_token().unwrap();
    }

    pub fn parse(&mut self) -> Result<JsonValue, LexError> {
        match self.current_token {
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array(),
            _ => Err(LexError::UnexpectedChar(
                self.current_token.to_string().chars().next().unwrap(),
                0,
            )),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, LexError> {
        let mut members = Vec::new();
        self.bump(); // Consume '{'

        while self.current_token != Token::RBrace {
            let key = match &self.current_token {
                Token::String(s) => s.clone(),
                _ => {
                    return Err(LexError::UnexpectedChar(
                        self.current_token.to_string().chars().next().unwrap(),
                        0,
                    ))
                }
            };

            self.bump(); // Consume key
            if self.current_token != Token::Colon {
                return Err(LexError::UnexpectedChar(':', 0));
            }
            self.bump(); // Consume ':'

            let value = self.parse_value()?;
            members.push((key, Box::new(value)));

            if self.current_token == Token::Comma {
                self.bump(); // Consume ',' and continue
            } else if self.current_token != Token::RBrace {
                return Err(LexError::UnexpectedChar(
                    self.current_token.to_string().chars().next().unwrap(),
                    0,
                ));
            }
        }

        self.bump(); // Consume '}'
        Ok(JsonValue::Object(members))
    }

    fn parse_array(&mut self) -> Result<JsonValue, LexError> {
        let mut elements = Vec::new();
        self.bump(); // Consume '['

        while self.current_token != Token::RBracket {
            let value = self.parse_value()?;
            elements.push(Box::new(value));

            if self.current_token == Token::Comma {
                self.bump(); // Consume ',' and continue
            } else if self.current_token != Token::RBracket {
                return Err(LexError::UnexpectedChar(
                    self.current_token.to_string().chars().next().unwrap(),
                    0,
                ));
            }
        }

        self.bump(); // Consume ']'
        Ok(JsonValue::Array(elements))
    }

    fn parse_value(&mut self) -> Result<JsonValue, LexError> {
        match &self.current_token {
            Token::String(s) => {
                let value = JsonValue::String(s.clone());
                self.bump(); // Consume the string
                Ok(value)
            }
            Token::Number(n) => {
                let value = JsonValue::Number(*n);
                self.bump(); // Consume the number
                Ok(value)
            }
            Token::Boolean(b) => {
                let value = JsonValue::Boolean(*b);
                self.bump(); // Consume the boolean
                Ok(value)
            }
            Token::Null => {
                self.bump(); // Consume null
                Ok(JsonValue::Null)
            }
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array(),
            _ => Err(LexError::UnexpectedChar(
                self.current_token.to_string().chars().next().unwrap(),
                0,
            )),
        }
    }
}
