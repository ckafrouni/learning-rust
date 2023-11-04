use std::collections::HashMap;

use crate::parser::*;
use crate::tokenizer::*;

use crate::parser::AstVisitor;

use super::values::Value;


pub struct AstInterpreter {
    pub stack: Vec<Value>,
    pub heap: HashMap<String, Value>,

}

impl AstVisitor for AstInterpreter {
    // TODO: imlement visitor trait
}