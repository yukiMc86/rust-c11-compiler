use crate::chibicch::{Function, Node, NodeKind};
use crate::utils::error;
use std::sync::atomic::{AtomicI32, Ordering};

static DEPTH: AtomicI32 = AtomicI32::new(0);

fn count() -> i32 {
    static I: AtomicI32 = AtomicI32::new(1);
    I.fetch_add(1, Ordering::SeqCst)
}

fn push() {
    DEPTH.fetch_add(1, Ordering::SeqCst);
    println!("  push %rax");
}

fn pop(str: &str) {
    DEPTH.fetch_sub(1, Ordering::SeqCst);
    println!("  pop {}", str);
}

// Compute the absolute address of a given node.
// It's an error if a given node does not reside in memory.
fn gen_addr(node: Box<Node>) {
    match node.kind {
        NodeKind::Var => {
            println!("  lea {}(%rbp), %rax", node.var.unwrap().offset);
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
            println!("  mov ${}, %rax", node.num.unwrap());
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

fn gen_stmt(node: Box<Node>) {
    match node.kind {
        NodeKind::ExprStmt => {
            gen_expr(node.lhs.unwrap());
        }
        NodeKind::Return => {
            gen_expr(node.lhs.unwrap());
            println!("  jmp .L.return");
        }
        NodeKind::Block => {
            let mut stmt_node = node.body;
            while let Some(mut n) = stmt_node {
                stmt_node = n.next.take();
                gen_stmt(n);
            }
        }
        NodeKind::If => {
            let c = count();

            gen_expr(node.cond.unwrap());
            println!("  cmp $0, %rax");
            println!("  je .L.else.{}", c);
            gen_stmt(node.then.unwrap());
            println!("  jmp .L.end.{}", c);
            println!(".L.else.{}:", c);
            if let Some(els) = node.els {
                gen_stmt(els);
            }
            println!(".L.end.{}:", c);
        }
        NodeKind::For => {
            let c = count();
            gen_stmt(node.init.unwrap());
            println!(".L.begin.{}:", c);

            if let Some(cond) = node.cond {
                gen_expr(cond);
                println!("  cmp $0, %rax");
                println!("  je .L.end.{}", c);
            }

            gen_stmt(node.then.unwrap());

            if let Some(inc) = node.inc {
                gen_expr(inc);
            }

            println!("  jmp .L.begin.{}", c);
            println!(".L.end.{}:", c);
        }
        _ => error("invalid statement"),
    }
}

pub fn codegen(prog: Function) {
    println!("  .global main");
    println!("main:");

    // Prologue
    println!("  push %rbp");
    println!("  mov %rsp, %rbp");
    println!("  sub ${}, %rsp\n", prog.stack_size);

    gen_stmt(prog.body);

    assert!(DEPTH.load(Ordering::SeqCst) == 0);

    println!(".L.return:");
    println!("  mov %rbp, %rsp");
    println!("  pop %rbp");
    println!("  ret");
}
