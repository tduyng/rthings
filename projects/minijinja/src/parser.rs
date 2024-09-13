use crate::{
    ast::{Ast, AstNode},
    macros::MacroRegistry,
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    macro_registry: MacroRegistry, // Handle macros within the parser
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Variable(String),
    OpenBlock,
    CloseBlock,
    If,
    Else,
    EndIf,
    For(String, String), // variable, collection
    EndFor,
    Macro(String, Vec<String>), // macro name, params
    EndMacro,
    Filter(String, String), // variable, filter name
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let tokens = lex(input);
        Parser {
            tokens,
            position: 0,
            macro_registry: MacroRegistry::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new();
        while let Some(token) = self.tokens.get(self.position) {
            match token {
                Token::Text(text) => ast.add_node(AstNode::Text(text.to_string())),
                Token::Variable(variable) => {
                    ast.add_node(AstNode::Variable(variable.to_string(), Vec::new()))
                }
                Token::OpenBlock => {
                    // Parse control structures like `if` and `for`
                    self.parse_block(&mut ast)?
                }
                _ => return Err(format!("Unexpected token: {:?}", token)),
            }
            self.position += 1;
        }
        Ok(ast)
    }

    fn parse_block(&mut self, ast: &mut Ast) -> Result<(), String> {
        self.position += 1; // Skip the block opening token

        let current_position = self.position; // Capture current position for later use

        match self.tokens.get(current_position) {
            Some(Token::If) => {
                self.position = current_position + 1;
                let condition_var =
                    if let Some(Token::Variable(var)) = self.tokens.get(self.position) {
                        self.position += 1;
                        var.to_string()
                    } else {
                        return Err("Expected variable after 'if'".to_string());
                    };

                let then_block = self.parse_block_body()?;

                let else_block = if let Some(Token::Else) = self.tokens.get(self.position) {
                    self.position += 1;
                    Some(Box::new(self.parse_block_body()?))
                } else {
                    None
                };

                ast.add_node(AstNode::IfBlock {
                    condition: condition_var,
                    then_block: Box::new(then_block),
                    else_block,
                });
            }

            _ => return Err("Unknown block type".to_string()),
        }
        Ok(())
    }

    fn parse_block_body(&mut self) -> Result<Ast, String> {
        let mut block_ast = Ast::new();
        while let Some(token) = self.tokens.get(self.position) {
            match token {
                Token::CloseBlock | Token::EndIf | Token::EndFor | Token::EndMacro => {
                    self.position += 1;
                    break;
                }
                _ => {
                    let node = self.parse_token()?;
                    block_ast.nodes.extend(node.nodes);
                }
            }
        }
        Ok(block_ast)
    }

    fn parse_token(&mut self) -> Result<Ast, String> {
        let mut node_ast = Ast::new();
        match self.tokens.get(self.position) {
            Some(Token::Text(text)) => {
                node_ast.add_node(AstNode::Text(text.to_string()));
                self.position += 1;
            }
            Some(Token::Variable(variable)) => {
                node_ast.add_node(AstNode::Variable(variable.to_string(), Vec::new()));
                self.position += 1;
            }
            _ => {
                return Err(format!(
                    "Unexpected token: {:?}",
                    self.tokens.get(self.position)
                ))
            }
        }
        Ok(node_ast)
    }
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next(); // consume second '{'
            let variable: String = chars.by_ref().take_while(|&ch| ch != '}').collect();
            tokens.push(Token::Variable(variable.trim().to_string()));
        } else if c == '{' && chars.peek() == Some(&'%') {
            chars.next(); // consume second '%'
            let block: String = chars.by_ref().take_while(|&ch| ch != '%').collect();
            tokens.push(lex_block(block.trim()));
        } else {
            tokens.push(Token::Text(c.to_string()));
        }
    }
    tokens
}

fn lex_block(input: &str) -> Token {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        ["if", _condition_var] => Token::If,
        ["for", var_name, "in", collection_var] => {
            Token::For(var_name.to_string(), collection_var.to_string())
        }
        ["macro", name, params @ ..] => Token::Macro(
            name.to_string(),
            params.iter().map(|s| s.to_string()).collect(),
        ),
        ["else"] => Token::Else,
        ["endif"] => Token::EndIf,
        ["endfor"] => Token::EndFor,
        ["endmacro"] => Token::EndMacro,
        _ => Token::OpenBlock, // fallback, should handle more cases
    }
}
