pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Variable(String),
    OpenBlock,
    CloseBlock,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let tokens = lex(input);
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new();
        while let Some(token) = self.tokens.get(self.position) {
            match token {
                Token::Text(text) => {
                    ast.add_node(AstNode::Text(text.to_string()));
                }
                Token::Variable(variable) => {
                    ast.add_node(AstNode::Variable(variable.to_string()));
                }
                _ => return Err("Unexpected token".to_string()),
            }
            self.position += 1;
        }
        Ok(ast)
    }
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut text_buffer = String::new();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next(); // consume second '{'

            if !text_buffer.is_empty() {
                tokens.push(Token::Text(text_buffer.clone()));
                text_buffer.clear();
            }

            let variable: String = chars.by_ref().take_while(|&ch| ch != '}').collect();
            if chars.next() == Some('}') {
                tokens.push(Token::Variable(variable.trim().to_string()));
            }
        } else {
            text_buffer.push(c);
        }
    }

    if !text_buffer.is_empty() {
        tokens.push(Token::Text(text_buffer));
    }

    tokens
}

pub struct Ast {
    pub nodes: Vec<AstNode>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: AstNode) {
        self.nodes.push(node)
    }
}

#[derive(Debug)]
pub enum AstNode {
    Text(String),
    Variable(String),
}
