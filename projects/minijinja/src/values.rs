#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Bool(b) => b.to_string(),
        }
    }
}
