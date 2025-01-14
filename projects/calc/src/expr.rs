#[derive(Debug, Clone)]
pub enum Value {
    Ident(String),
    Number(f64),
    String(String),
}

impl Value {
    pub fn add(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            _ => None,
        }
    }

    pub fn sub(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }

    pub fn mul(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            _ => None,
        }
    }

    pub fn div(self, other: Value) -> Option<Value> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if b != 0.0 {
                    Some(Value::Number(a / b))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
