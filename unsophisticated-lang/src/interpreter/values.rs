#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Char(char),
    Symbol(String),
}

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs + rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs.to_string() + rhs)),
            _ => Err(format!("Cannot add {:?} and {:?}", self, other)),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs - rhs)),
            _ => Err(format!("Cannot sub {:?} and {:?}", self, other)),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs * rhs)),
            _ => Err(format!("Cannot mul {:?} and {:?}", self, other)),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs / rhs)),
            _ => Err(format!("Cannot div {:?} and {:?}", self, other)),
        }
    }

    pub fn neg(&self) -> Result<Value, String> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(format!("Cannot neg {:?}", self)),
        }
    }

    pub fn not(&self) -> Result<Value, String> {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(format!("Cannot not {:?}", self)),
        }
    }

    // -- region : helper functions

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => false,
        }
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::Char(c)
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
