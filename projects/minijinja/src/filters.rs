use crate::values::Value;

pub fn apply_filter(name: &str, value: Value) -> Value {
    match name {
        "upper" => match value {
            Value::String(s) => Value::String(s.to_uppercase()),
            _ => value,
        },
        "lower" => match value {
            Value::String(s) => Value::String(s.to_lowercase()),
            _ => value,
        },
        _ => value,
    }
}
