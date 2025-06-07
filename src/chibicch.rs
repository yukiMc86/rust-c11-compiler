use crate::utils::{error, error_at};

#[derive(PartialEq)]
pub enum TokenKind {
    Empty,
    Ident, // Identifiers
    Punct,
    Num,
    EOF,
}

/// Token type
pub struct Token {
    pub kind: TokenKind,
    pub num: Option<i32>,
    pub string: Option<String>,
    pub next: Option<Box<Token>>,
    pub location: usize,
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
    ExprStmt, // Expression statement
    Var,      // Variable
    Num,      // Integer
}

/// AST node type
pub struct Node {
    pub kind: NodeKind,
    pub next: Option<Box<Node>>,
    pub val: Option<i32>,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub name: Option<String>,
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
        if self.kind != TokenKind::Punct {
            false
        } else if let Some(ref string) = self.string {
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
            next: None,
            val: None,
            lhs: None,
            rhs: None,
            name: None,
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
        node.val = Some(val);
        node
    }

    pub fn new_unary(kind: NodeKind, expr: Box<Node>) -> Box<Node> {
        let mut node = Node::new(kind);
        node.lhs = Some(expr);
        node
    }

    pub fn new_var(name: String) -> Box<Node> {
        let mut node = Node::new(NodeKind::Var);
        node.name = Some(name);
        node
    }

    pub fn next(self) -> Box<Node> {
        self.next.unwrap()
    }

    pub fn next_mut(&mut self) -> &mut Box<Node> {
        self.next.as_mut().unwrap()
    }
}
