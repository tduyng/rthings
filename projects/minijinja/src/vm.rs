use std::collections::HashMap;

use crate::{
    ast::{Ast, AstNode},
    filters::FilterRegistry,
    macros::MacroRegistry,
    values::Value,
};

pub struct Vm {
    variables: HashMap<String, Value>,
    macros: MacroRegistry,
    filters: FilterRegistry,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            variables: HashMap::new(),
            macros: MacroRegistry::new(),
            filters: FilterRegistry::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn set_macro_registry(&mut self, macros: MacroRegistry) {
        self.macros = macros;
    }

    pub fn set_filter_registry(&mut self, filters: FilterRegistry) {
        self.filters = filters;
    }

    pub fn render(&mut self, ast: &Ast) -> Result<String, String> {
        let mut output = String::new();
        for node in &ast.nodes {
            match node {
                AstNode::Text(text) => output.push_str(&text),
                AstNode::Variable(variable, filters) => {
                    let value = self.evaluate_variable(variable)?;
                    let filtered_value = self.apply_filters(value, filters)?;
                    output.push_str(&filtered_value.to_string());
                }
                AstNode::IfBlock {
                    condition,
                    then_block,
                    else_block,
                } => {
                    let condition_value = self.evaluate_variable(condition)?;
                    if self.is_truthy(&condition_value) {
                        output.push_str(&self.render(then_block)?);
                    } else if let Some(else_block) = else_block {
                        output.push_str(&self.render(else_block)?);
                    }
                }
                AstNode::ForBlock {
                    loop_var,
                    collection_var,
                    body,
                } => {
                    let collection_value = self.evaluate_variable(collection_var)?;
                    if let Value::List(list) = collection_value {
                        for item in list {
                            self.variables.insert(loop_var.clone(), item.clone());
                            output.push_str(&self.render(body)?);
                        }
                    } else {
                        return Err(format!(
                            "Expected a list for '{}' but found {:?}",
                            collection_var, collection_value
                        ));
                    }
                }
                AstNode::MacroCall { name, args } => {
                    let macro_args: Vec<Value> = args
                        .iter()
                        .map(|arg| self.evaluate_variable(arg))
                        .collect::<Result<_, _>>()?;
                    let mut vm = self.clone();
                    if let Err(err) = self.macros.call_macro(name, macro_args, &mut vm) {
                        return Err(err);
                    }
                }
            }
        }
        Ok(output)
    }

    fn evaluate_variable(&self, var_name: &str) -> Result<Value, String> {
        self.variables
            .get(var_name)
            .cloned()
            .ok_or_else(|| format!("Variable {} not found", var_name))
    }

    fn apply_filters(
        &self,
        value: Value,
        filters: &[(String, Vec<Value>)],
    ) -> Result<String, String> {
        let mut result = value;
        for (filter_name, _args) in filters {
            if let Err(e) = self.filters.apply_filter(&filter_name, &result) {
                return Err(e);
            }
            result = self.filters.apply_filter(&filter_name, &result)?;
        }
        Ok(result.to_string())
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Dict(o) => !o.is_empty(),
            Value::None => false,
        }
    }
}

impl Clone for Vm {
    fn clone(&self) -> Vm {
        Vm {
            variables: self.variables.clone(),
            macros: self.macros.clone(),
            filters: self.filters.clone(),
        }
    }
}
