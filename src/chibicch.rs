use crate::utils::{error, error_at};

#[derive(PartialEq)]
pub enum TokenKind {
    Empty,
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
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Neg, // Unary -
    Eq,  // ==
    Ne,  // !=
    Lt,  // <
    Le,  // <=
    Num, // Integer
}

/// AST node type
pub struct Node {
    pub kind: NodeKind,
    pub val: Option<i32>,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
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
        match self.next {
            Some(token) => token,
            None => error("cannot call next on a non-empty token"),
        }
    }

    pub fn next_mut(&mut self) -> &mut Box<Token> {
        match self.next {
            Some(ref mut token) => token,
            None => error("cannot call next_mut on a non-empty token"),
        }
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
    pub fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        Box::new(Node {
            kind,
            val: None,
            lhs: Some(lhs),
            rhs: Some(rhs),
        })
    }

    pub fn new_num(val: i32) -> Box<Node> {
        Box::new(Node {
            kind: NodeKind::Num,
            val: Some(val),
            lhs: None,
            rhs: None,
        })
    }

    pub fn new_unary(kind: NodeKind, expr: Box<Node>) -> Box<Node> {
        Box::new(Node {
            kind,
            val: None,
            lhs: Some(expr),
            rhs: None,
        })
    }
}
