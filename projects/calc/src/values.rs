use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Ident(String),
}

#[derive(Debug)]
pub enum ValueError {
    TypeMismatch,
    DivisionByZero,
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueError::TypeMismatch => write!(f, "Type mismatch error"),
            ValueError::DivisionByZero => write!(f, "Division by zero error"),
        }
    }
}

impl Value {
    pub fn add(self, other: Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
            (Value::String(left), Value::String(right)) => Ok(Value::String(left + &right)),
            _ => Err(ValueError::TypeMismatch),
        }
    }

    pub fn sub(self, other: Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left - right)),
            _ => Err(ValueError::TypeMismatch),
        }
    }

    pub fn mul(self, other: Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left * right)),
            _ => Err(ValueError::TypeMismatch),
        }
    }

    pub fn div(self, other: Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => {
                if right == 0.0 {
                    Err(ValueError::DivisionByZero)
                } else {
                    Ok(Value::Number(left / right))
                }
            }
            _ => Err(ValueError::TypeMismatch),
        }
    }
}
