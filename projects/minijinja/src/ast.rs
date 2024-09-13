use crate::values::Value;

#[derive(Clone)]
pub enum AstNode {
    Text(String),
    Variable(String, Vec<(String, Vec<Value>)>), // Variable name and a list of filters (filter name, args)
    IfBlock {
        condition: String,
        then_block: Box<Ast>,
        else_block: Option<Box<Ast>>,
    },
    ForBlock {
        loop_var: String,
        collection_var: String,
        body: Box<Ast>,
    },
    MacroCall {
        name: String,
        args: Vec<String>,
    },
}

#[derive(Clone)]
pub struct Ast {
    pub nodes: Vec<AstNode>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: AstNode) {
        self.nodes.push(node);
    }
}
