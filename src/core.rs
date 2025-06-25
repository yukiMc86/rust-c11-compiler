use crate::utils::{error, error_at};
use std::cell::RefCell;
use std::rc::Rc;

type OptRcRef<T> = Option<Rc<RefCell<T>>>;

#[derive(PartialEq)]
pub enum TokenKind {
    Empty,
    Ident,    // Identifiers
    Keywords, // Keywords
    Punct,    // Punctuators
    Num,      // Numeric literals
    EOF,      // End-of-file markers
}

/// Token type
pub struct Token {
    pub kind: TokenKind,
    pub num: Option<i32>,
    pub string: Option<String>,
    pub next: Option<Box<Token>>,
    pub location: usize,
}

//
// parse.c
//

// Local variable
#[derive(Clone)]
pub struct Obj {
    pub name: String, // Variable name
    pub offset: i32,  // Offset from RBP
}

// Function
pub struct Function {
    pub body: Box<Node>,
    pub _locals: Vec<Obj>,
    pub stack_size: i32,
}

#[derive(PartialEq)]
pub enum NodeKind {
    Empty,    // Empty node
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Neg,      // Unary -
    Eq,       // ==
    Ne,       // !=
    Lt,       // <
    Le,       // <=
    Assign,   // =
    Return,   // "return"
    If,       // "if"
    For,      // "for" or "while"
    Block,    // { ... }
    ExprStmt, // Expression statement
    Var,      // Variable
    Num,      // Integer
}

/// AST node type
#[derive(Default)]
pub struct Node {
    pub kind: NodeKind,          // Node kind
    pub next: Option<Box<Node>>, // Next node

    pub lhs: Option<Box<Node>>, // Left-hand side
    pub rhs: Option<Box<Node>>, // Right-hand side

    // "if" or "for" statement
    pub cond: Option<Box<Node>>, // Condition
    pub then: Option<Box<Node>>, // Then branch
    pub els: Option<Box<Node>>,  // Else branch
    pub init: Option<Box<Node>>,
    pub inc: Option<Box<Node>>,

    pub body: Option<Box<Node>>, // Block
    pub var: Option<Obj>,        // Used if kind == ND_VAR
    pub num: Option<i32>,        // Used if kind == ND_NUM
}

impl Token {
    pub fn new_token(kind: TokenKind, location: usize) -> Box<Token> {
        Box::new(Token {
            kind,
            num: None,
            string: None,
            next: None,
            location,
        })
    }

    pub fn next(self) -> Box<Token> {
        self.next.unwrap()
    }

    pub fn next_mut(&mut self) -> &mut Box<Token> {
        self.next.as_mut().unwrap()
    }

    pub fn push(&mut self, token: Box<Token>) {
        match self.next {
            None => self.next = Some(token),
            _ => error("cannot push to a non-empty token"),
        }
    }

    pub fn eq_punct(&self, s: &str) -> bool {
        if let Some(ref string) = self.string {
            string == s
        } else {
            false
        }
    }

    pub fn skip(self, str: &str) -> Box<Token> {
        if !self.eq_punct(str) {
            error_at(self.location, &format!("expected a '{}'", str));
        }
        self.next()
    }
}

impl Node {
    pub fn new(kind: NodeKind) -> Box<Node> {
        Box::new(Node {
            kind,
            ..Default::default()
        })
    }

    pub fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        let mut node = Node::new(kind);
        node.lhs = Some(lhs);
        node.rhs = Some(rhs);
        node
    }

    pub fn new_num(val: i32) -> Box<Node> {
        let mut node = Node::new(NodeKind::Num);
        node.num = Some(val);
        node
    }

    pub fn new_unary(kind: NodeKind, expr: Box<Node>) -> Box<Node> {
        let mut node = Node::new(kind);
        node.lhs = Some(expr);
        node
    }

    pub fn new_var(var: Obj) -> Box<Node> {
        let mut node = Node::new(NodeKind::Var);
        node.var = Some(var);
        node
    }

    pub fn next(self) -> Box<Node> {
        self.next.unwrap()
    }

    pub fn next_mut(&mut self) -> &mut Box<Node> {
        self.next.as_mut().unwrap()
    }
}

impl Default for NodeKind {
    fn default() -> Self {
        NodeKind::Empty
    }
}
