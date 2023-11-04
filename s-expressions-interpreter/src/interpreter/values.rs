#[derive(Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Symbol(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<Value> for bool {
    fn from(v: Value) -> Self {
        match v {
            Value::Bool(b) => b,
            _ => panic!("Cannot convert {:?} to bool", v),
        }
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        match v {
            Value::Number(n) => n,
            _ => panic!("Cannot convert {:?} to f64", v),
        }
    }
}

impl From<Value> for String {
    fn from(v: Value) -> Self {
        match v {
            Value::String(s) => s,
            _ => panic!("Cannot convert {:?} to String", v),
        }
    }
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Symbol(s) => write!(f, "{}", s),
        }
    }
}