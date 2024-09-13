use std::collections::HashMap;

use crate::parser::AstNode;

pub struct Macro {
    name: String,
    body: String,
}

impl Macro {
    pub fn new(name: &str, body: &str) -> Macro {
        Macro {
            name: name.to_string(),
            body: body.to_string(),
        }
    }

    pub fn render(&self) -> String {
        self.body.clone()
    }
}

pub struct MacroRegistry {
    macros: HashMap<String, Macro>,
}

impl MacroRegistry {
    pub fn new() -> MacroRegistry {
        MacroRegistry {
            macros: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, body: &str) {
        self.macros.insert(name.to_string(), Macro::new(name, body));
    }

    pub fn get(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }
}
