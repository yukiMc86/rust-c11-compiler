use crate::chibicch::{Function, Node, NodeKind, Obj, Token, TokenKind};
use crate::utils::error_at;
use std::sync::Mutex;

/// All local variable instances created during parsing are
///
/// accumulated to this list.
static LOCALS_VAR: Mutex<Option<Vec<Obj>>> = Mutex::new(None);

fn new_locals_var() {
    let mut locals = LOCALS_VAR.lock().unwrap();
    *locals = Some(Vec::new());
}

fn push_local_var(var: Obj) {
    let mut locals = LOCALS_VAR.lock().unwrap();
    locals.as_mut().unwrap().push(var);
}

fn get_locals_var() -> Vec<Obj> {
    let mut locals = LOCALS_VAR.lock().unwrap();
    locals.take().unwrap()
}

fn get_offset() -> i32 {
    let locals = LOCALS_VAR.lock().unwrap();
    -(locals.as_ref().unwrap().len() as i32) * 8
}

fn find_var(name: &str) -> Option<Obj> {
    let locals = LOCALS_VAR.lock().unwrap();
    locals
        .as_ref()
        .unwrap()
        .iter()
        .find(|&v| v.name == name)
        .cloned()
}

/// Round up `n` to the nearest multiple of `align`. For instance,0
///
/// align_to(5, 8) returns 8 and align_to(11, 8) returns 16.
fn align_to(align: i32) -> i32 {
    let locals = LOCALS_VAR.lock().unwrap();
    let offset = locals.as_ref().unwrap().len() as i32 * 8;
    offset + (align - 1) / align * align
}

/// stmt = "return" expr ";"
///      | "{" compound-stmt
///      | expr-stmt
fn stmt(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    if token.eq_punct("return") {
        let (expr_node, next_token) = expr(token.next());
        let node = Node::new_unary(NodeKind::Return, expr_node);
        return (node, next_token.skip(";"));
    }

    if token.eq_punct("{") {
        return compound_stmt(token.next());
    }

    expr_stmt(token)
}

/// compound-stmt = stmt* "}"
fn compound_stmt(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let mut head = Node::new(NodeKind::Empty);
    let mut current = &mut head;

    let mut next_token = token;
    let mut expr_node: Box<Node>;
    while !(&next_token).eq_punct("}") {
        (expr_node, next_token) = stmt(next_token);
        current.next = Some(expr_node);
        current = current.next_mut();
    }

    let mut node = Node::new(NodeKind::Block);
    node.body = Some(head.next());
    next_token = next_token.next(); // Skip the closing brace '}'

    (node, next_token)
}

/// expr-stmt = expr? ";"
fn expr_stmt(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    if (&token).eq_punct(";") {
        return (Node::new(NodeKind::Block), token.next());
    }

    let (expr_node, next_token) = expr(token);
    let node = Node::new_unary(NodeKind::ExprStmt, expr_node);
    return (node, next_token.skip(";"));
}

// expr = assign
fn expr(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    return assign(token);
}

fn assign(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let mut left_node: Box<Node>;
    let mut next_token: Box<Token>;
    (left_node, next_token) = equality(token);

    if next_token.eq_punct("=") {
        next_token = next_token.next();
        let (right_node, token) = assign(next_token);
        left_node = Node::new_binary(NodeKind::Assign, left_node, right_node);
        next_token = token;
    }

    return (left_node, next_token);
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

// primary = "(" expr ")" | ident | num
fn primary(token: Box<Token>) -> (Box<Node>, Box<Token>) {
    let node: Box<Node>;
    let mut next_token: Box<Token>;

    if token.eq_punct("(") {
        next_token = token.next();
        (node, next_token) = expr(next_token);
        next_token = next_token.skip(")");
        return (node, next_token);
    }

    if token.kind == TokenKind::Ident {
        let name = token.string.clone().unwrap();
        let var: Obj;
        if let Some(exist_var) = find_var(name.as_str()) {
            var = Obj {
                name,
                offset: exist_var.offset,
            };
        } else {
            var = Obj {
                name,
                offset: get_offset(),
            };
            push_local_var(var.clone());
        }

        node = Node::new_var(var);
        next_token = token.next();
        return (node, next_token);
    }

    if token.kind == TokenKind::Num {
        node = Node::new_num(token.num.unwrap());
        next_token = token.next();
        return (node, next_token);
    }

    error_at(token.location, "expected an expression");
}

pub fn parse(mut token: Box<Token>) -> Function {
    token = token.skip("{");

    new_locals_var();

    let body = compound_stmt(token).0;

    let stack_size = align_to(16);
    let _locals = get_locals_var();

    Function {
        body,
        _locals,
        stack_size,
    }
}
