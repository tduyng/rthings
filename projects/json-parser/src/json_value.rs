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

impl JsonValue {
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        if let JsonValue::Object(entries) = self {
            for (k, v) in entries {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }

    pub fn set(&mut self, key: String, value: JsonValue) {
        if let JsonValue::Object(entries) = self {
            for (k, v) in entries.iter_mut() {
                if k == &key {
                    *v = Box::new(value);
                    return;
                }
            }
            entries.push((key, Box::new(value)));
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            JsonValue::Object(entries) => {
                let entries_str: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_json()))
                    .collect();
                format!("{{{}}}", entries_str.join(", "))
            }
            JsonValue::Array(elements) => {
                let elements_str: Vec<String> = elements.iter().map(|v| v.to_json()).collect();
                format!("[{}]", elements_str.join(", "))
            }
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Null => "null".to_string(),
        }
    }
}
