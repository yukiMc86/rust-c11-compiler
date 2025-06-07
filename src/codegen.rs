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

// Compute the absolute address of a given node.
// It's an error if a given node does not reside in memory.
fn gen_addr(node: Box<Node>) {
    match node.kind {
        NodeKind::Var => {
            let offset =
                (node.name.as_ref().unwrap().chars().next().unwrap() as i32 - 'a' as i32) * 8;

            println!("  lea {}(%rbp), %rax", -offset);
        }
        _ => error("not an lvalue"),
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
        NodeKind::Var => {
            gen_addr(node);
            println!("  mov (%rax), %rax");
            return;
        }
        NodeKind::Assign => {
            gen_addr(node.lhs.unwrap());
            push();
            gen_expr(node.rhs.unwrap());
            pop("%rdi");
            println!("  mov %rax, (%rdi)");
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

    // Prologue
    println!("  push %rbp");
    println!("  mov %rsp, %rbp");
    println!("  sub $208, %rsp");

    let mut stmt_node = Some(node);

    while let Some(n) = stmt_node {
        stmt_node = gen_stmt(n);
        assert!(unsafe { DEPTH } == 0);
    }

    println!("  mov %rbp, %rsp");
    println!("  pop %rbp");
    println!("  ret");
}
