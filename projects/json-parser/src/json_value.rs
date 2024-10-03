use std::fmt;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(Vec<(String, Box<JsonValue>)>),
    Array(Vec<Box<JsonValue>>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Object(entries) => {
                let entries_str: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v))
                    .collect();
                write!(f, "{{{}}}", entries_str.join(", "))
            }
            JsonValue::Array(elements) => {
                let elements_str: Vec<String> = elements.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", elements_str.join(", "))
            }
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
        }
    }
}
