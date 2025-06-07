use crate::chibicch::{Node, NodeKind, Token, TokenKind};
use crate::utils::error_at;

// stmt = expr-stmt
fn stmt(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    expr_stmt(token)
}

// expr-stmt = expr ";"
fn expr_stmt(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let (left_node, next_token) = expr(token);
    let node = Node::new_unary(NodeKind::ExprStmt, left_node);
    let next_token = next_token.skip(";");
    return (node, next_token);
}

// expr = equality
fn expr(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    return equality(token);
}

// equality = relational ("==" relational | "!=" relational)*
fn equality(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let mut left_node: Box<Node>;
    let mut next_token: Box<Token>;
    (left_node, next_token) = relational(token);

    loop {
        if next_token.eq_punct("==") {
            next_token = next_token.next();
            let (right_node, token) = relational(next_token);
            left_node = Node::new_binary(NodeKind::Eq, left_node, right_node);
            next_token = token;
            continue;
        }

        if next_token.eq_punct("!=") {
            next_token = next_token.next();
            let (right_node, token) = relational(next_token);
            left_node = Node::new_binary(NodeKind::Ne, left_node, right_node);
            next_token = token;
            continue;
        }

        return (left_node, next_token);
    }
}

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn relational(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let mut left_node: Box<Node>;
    let mut next_token: Box<Token>;
    (left_node, next_token) = add(token);

    loop {
        if next_token.eq_punct("<") {
            next_token = next_token.next();
            let (right_node, token) = add(next_token);
            left_node = Node::new_binary(NodeKind::Lt, left_node, right_node);
            next_token = token;
            continue;
        }

        if next_token.eq_punct("<=") {
            next_token = next_token.next();
            let (right_node, token) = add(next_token);
            left_node = Node::new_binary(NodeKind::Le, left_node, right_node);
            next_token = token;
            continue;
        }

        if next_token.eq_punct(">") {
            next_token = next_token.next();
            let (right_node, token) = equality(next_token);
            left_node = Node::new_binary(NodeKind::Lt, right_node, left_node);
            next_token = token;
            continue;
        }

        if next_token.eq_punct(">=") {
            next_token = next_token.next();
            let (right_node, token) = equality(next_token);
            left_node = Node::new_binary(NodeKind::Le, right_node, left_node);
            next_token = token;
            continue;
        }

        return (left_node, next_token);
    }
}

// add = mul ("+" mul | "-" mul)*
fn add(token: Box<Token>) -> (Box<Node>, Box<Token>) {
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
fn mul(token: Box<Token>) -> (Box<Node>, Box<Token>) {
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
fn unary(token: Box<Token>) -> (Box<Node>, Box<Token>) {
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
fn primary(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let node: Box<Node>;
    let mut next_token: Box<Token>;

    if token.eq_punct("(") {
        next_token = token.next();
        (node, next_token) = expr(next_token);
        next_token = next_token.skip(")");
        return (node, next_token);
    }

    if token.kind == TokenKind::Num {
        node = Node::new_num(token.num.unwrap());
        next_token = token.next();
        return (node, next_token);
    }

    error_at(token.location, "expected an expression");
}

pub fn parse(mut token: Box<Token>) -> Box<Node> {
    let mut head = Node {
        kind: NodeKind::Empty,
        next: None,
        val: None,
        lhs: None,
        rhs: None,
    };

    let mut current = &mut head;

    while token.kind != TokenKind::EOF {
        let (node, next_token) = stmt(token);
        current.next = Some(node);
        current = current.next_mut();
        token = next_token;
    }

    head.next()
}
