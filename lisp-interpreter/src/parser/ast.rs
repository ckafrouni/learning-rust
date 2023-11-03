use crate::tokenizer::ReservedKeyword;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Node {
        kind: AstKind,
        children: Vec<AstNode>,
    },
    Leaf {
        kind: AstKind,
    },
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
    FnCall,

    Reserved(ReservedKeyword),

    Prog,
}

impl AstNode {
    pub fn new_node(kind: AstKind) -> Self {
        AstNode::Node {
            kind,
            children: Vec::new(),
        }
    }

    pub fn new_leaf(kind: AstKind) -> Self {
        AstNode::Leaf { kind }
    }

    pub fn add_child(&mut self, node: AstNode) {
        if let AstNode::Node { children, .. } = self {
            children.push(node);
        } else {
            panic!("Cannot add child to a leaf node!");
        }
    }

    // pub fn kind(&self) -> &AstKind {
    //     match self {
    //         AstNode::Node { kind, .. } => kind,
    //         AstNode::Leaf { kind } => kind,
    //     }
    // }
}
