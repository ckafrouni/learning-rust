use crate::tokenizer::ReservedKeyword;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    // BinaryOp {
    //     op: Operator,
    //     lhs: Box<AstNode>,
    //     rhs: Box<AstNode>,
    // },
    // UnaryOp {
    //     op: Operator,
    //     expr: Box<AstNode>,
    // },

    // TODO: Separate out the different types of nodes
    // Example:
    //      - AstNode::FnCall { name: String, args: Vec<AstNode> }
    //      - AstNode::If { cond: AstNode, then: AstNode, else: AstNode }
    //      - AstNode::Let { name: String, value: AstNode }
    //      - AstNode::Def { name: String, value: AstNode }
    //      - AstNode::Ident { name: String }
    //      - AstNode::Literal { value: Literal } (Literal::Number, Literal::String, Literal::Nil, Literal::Bool, etc.)
    //      - AstNode::Binary { op: BinaryOp, lhs: AstNode, rhs: AstNode }
    //      - AstNode::Unary { op: UnaryOp, expr: AstNode }
    //      - AstNode::Prog { exprs: Vec<AstNode> }
    Node {
        kind: AstKind,
        children: Vec<AstNode>,
    },
    Leaf {
        kind: AstKind,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Not,
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

/// Implement Display for AstNode
/// Example:
///      - AstNode::FnCall { name: String, args: Vec<AstNode> }
impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AstNode::Node { kind, children } => {
                write!(f, "({}", kind)?;
                for child in children {
                    write!(f, " {}", child)?;
                }
                write!(f, ")")
            }
            AstNode::Leaf { kind } => write!(f, "{}", kind),
        }
    }
}

/// Implement Display for AstKind
/// Example:
///     - AstNode::FnCall { name: String, args: Vec<AstNode> }
impl std::fmt::Display for AstKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AstKind::Number(n) => write!(f, "{}", n),
            AstKind::String(s) => write!(f, "\"{}\"", s),
            AstKind::Ident(s) => write!(f, "{}", s),
            AstKind::Add => write!(f, "+"),
            AstKind::Sub => write!(f, "-"),
            AstKind::Mul => write!(f, "*"),
            AstKind::Div => write!(f, "/"),
            AstKind::Nil => write!(f, "nil"),
            AstKind::Neg => write!(f, "-"),
            AstKind::Not => write!(f, "!"),
            AstKind::FnCall => write!(f, "fn-call"),
            AstKind::Reserved(kw) => write!(f, "{}", kw),
            AstKind::Prog => write!(f, "prog"),
        }
    }
}
