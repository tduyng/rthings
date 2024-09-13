use std::collections::HashMap;

use crate::{
    parser::{Ast, AstNode},
    values::Value,
};

pub struct Vm;

impl Vm {
    pub fn new() -> Vm {
        Vm
    }

    pub fn execute(&self, ast: &Ast, context: &Context) -> String {
        let mut output = String::new();
        for node in &ast.nodes {
            match node {
                AstNode::Text(text) => output.push_str(&text),
                AstNode::Variable(variable) => {
                    if let Some(value) = context.get(variable) {
                        output.push_str(&value.to_string());
                    } else {
                        output.push_str("{{ Unknown variable }}");
                    }
                }
            }
        }
        output
    }
}

pub struct Context {
    variables: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.variables.get(key)
    }
}
