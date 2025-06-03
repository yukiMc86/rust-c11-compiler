use crate::{
    core::token::{Token, TokenKind},
    pop, push,
    utils::{error, error_at},
};

#[derive(PartialEq)]
pub enum NodeKind {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Neg, // Unary -
    Num, // Integer
}

pub struct Node {
    pub kind: NodeKind,
    pub val: Option<i32>,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
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

// expr = mul ("+" mul | "-" mul)*
pub fn expr(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let mut left_node: Box<Node>;
    let mut next_token: Box<Token>;

    (left_node, next_token) = mul(token);

    loop {
        if next_token.eq_punct("+") {
            next_token = next_token.next();
            let (right_node, token) = mul(next_token);
            left_node = Node::new_binary(NodeKind::Add, left_node, right_node);
            next_token = token;
            continue;
        }

        if next_token.eq_punct("-") {
            next_token = next_token.next();
            let (right_node, token) = mul(next_token);
            left_node = Node::new_binary(NodeKind::Sub, left_node, right_node);
            next_token = token;
            continue;
        }

        return (left_node, next_token);
    }
}

// mul = unary ("*" unary | "/" unary)*
pub fn mul(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let mut left_node: Box<Node>;
    let mut next_token: Box<Token>;

    (left_node, next_token) = unary(token);

    loop {
        if next_token.eq_punct("*") {
            next_token = next_token.next();
            let (right_node, token) = unary(next_token);
            left_node = Node::new_binary(NodeKind::Mul, left_node, right_node);
            next_token = token;
            continue;
        }

        if next_token.eq_punct("/") {
            next_token = next_token.next();
            let (right_node, token) = unary(next_token);
            left_node = Node::new_binary(NodeKind::Div, left_node, right_node);
            next_token = token;
            continue;
        }

        return (left_node, next_token);
    }
}

// unary = ("+" | "-") unary
//       | primary
pub fn unary(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    if token.eq_punct("+") {
        return unary(token.next());
    }

    if token.eq_punct("-") {
        let (expr_node, next_token) = unary(token.next());
        return (Node::new_unary(NodeKind::Neg, expr_node), next_token);
    }

    primary(token)
}

// primary = "(" expr ")" | num
pub fn primary(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let node: Box<Node>;
    let mut next_token: Box<Token>;

    if token.eq_punct("(") {
        next_token = token.next();
        (node, next_token) = expr(next_token);
        next_token = next_token.skip(")");
        return (node, next_token);
    }

    if token.kind == TokenKind::Num {
        node = Node::new_num(token.get_number());
        next_token = token.next();
        return (node, next_token);
    }

    error_at(token.location, "expected an expression");
}

pub fn gen_expr(node: Box<Node>) {
    match node.kind {
        NodeKind::Neg => {
            gen_expr(node.lhs.unwrap());
            println!("  neg %rax");
            return;
        }
        NodeKind::Num => {
            println!("  mov ${}, %rax", node.val.unwrap());
            return;
        }
        _ => {}
    }

    gen_expr(node.rhs.unwrap());
    push();
    gen_expr(node.lhs.unwrap());
    pop("%rdi");

    match node.kind {
        NodeKind::Add => println!("  add %rdi, %rax"),
        NodeKind::Sub => println!("  sub %rdi, %rax"),
        NodeKind::Mul => println!("  imul %rdi, %rax"),
        NodeKind::Div => {
            println!("  cqo");
            println!("  idiv %rdi");
        }
        _ => error("invalid expression"),
    }
}
