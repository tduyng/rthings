use crate::values::Value;
use std::collections::HashMap;

pub type FilterFn = fn(&Value) -> Result<Value, String>;

pub struct FilterRegistry {
    filters: HashMap<String, FilterFn>,
}

impl FilterRegistry {
    pub fn new() -> FilterRegistry {
        let mut registry = FilterRegistry {
            filters: HashMap::new(),
        };

        registry.add_filter("upper", |v| match v {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => Err("upper filter only works on strings".to_string()),
        });

        registry.add_filter("lower", |v| match v {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            _ => Err("lower filter only works on strings".to_string()),
        });

        registry
    }

    pub fn add_filter(&mut self, name: &str, filter: FilterFn) {
        self.filters.insert(name.to_string(), filter);
    }

    pub fn apply_filter(&self, name: &str, value: &Value) -> Result<Value, String> {
        if let Some(filter) = self.filters.get(name) {
            filter(value)
        } else {
            Err(format!("Filter {} not found", name))
        }
    }
}

pub fn apply_filter(name: &str, value: &Value) -> Result<Value, String> {
    let registry = FilterRegistry::new();
    registry.apply_filter(name, value)
}

impl Clone for FilterRegistry {
    fn clone(&self) -> FilterRegistry {
        FilterRegistry {
            filters: self.filters.clone(),
        }
    }
}
