use core::fmt;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    None,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::List(list) => write!(f, "{:?}", list),
            Value::Dict(dict) => write!(f, "{{{:?}}}", dict),
            Value::None => write!(f, "None"),
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::Float(fl) => fl.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::List(list) => format!("{:?}", list),
            Value::Dict(dict) => format!("{:?}", dict),
            Value::None => "None".to_string(),
        }
    }
}
