use crate::chibicch::{Node, NodeKind};
use crate::utils::error;

static mut DEPTH: i32 = 0;

fn push() {
    unsafe {
        DEPTH += 1;
        println!("  push %rax");
    }
}

fn pop(str: &str) {
    unsafe {
        DEPTH -= 1;
        println!("  pop {}", str);
    }
}

fn gen_expr(node: Box<Node>) {
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
        NodeKind::Eq | NodeKind::Ne | NodeKind::Lt | NodeKind::Le => {
            println!("  cmp %rdi, %rax");
            match node.kind {
                NodeKind::Eq => println!("  sete %al"),
                NodeKind::Ne => println!("  setne %al"),
                NodeKind::Lt => println!("  setl %al"),
                NodeKind::Le => println!("  setle %al"),
                _ => error("invalid comparison operator"),
            }
            println!("  movzb %al, %rax");
        }
        _ => error("invalid expression"),
    }
}

fn gen_stmt(node: Box<Node>) -> Option<Box<Node>> {
    match node.kind {
        NodeKind::ExprStmt => {
            gen_expr(node.lhs.unwrap());
            node.next
        }
        _ => error("invalid statement"),
    }
}

pub fn codegen(node: Box<Node>) {
    println!("  .global main");
    println!("main:");

    let mut stmt_node = Some(node);

    while let Some(n) = stmt_node {
        stmt_node = gen_stmt(n);
        assert!(unsafe { DEPTH } == 0);
    }

    println!("  ret");
}
