
#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    pub kind: AstKind,
    pub children: Vec<AstNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstLeaf {
    pub kind: AstKind,
}


#[derive(Debug, Clone, PartialEq)]
pub enum AstKind {
    Number(i32),
    String(String),
    Ident(String),
    Add,
    Sub,
    Mul,
    Div,
    Nil,
    Neg,
    Not,
    Call,
    Def,
}

impl AstNode {
    pub fn new(kind: AstKind) -> Self {
        Self {
            kind,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: AstNode) {
        self.children.push(node);
    }
}

impl AstLeaf {
    pub fn new(kind: AstKind) -> Self {
        Self { kind }
    }
}