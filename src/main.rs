mod core;
mod utils;

use core::{
    node::{expr, gen_expr},
    token::{TokenKind, tokenize},
};
use once_cell::sync::OnceCell;
use std::env;
use utils::{error, error_at};

pub static CURRENT_INPUT: OnceCell<String> = OnceCell::new();

static mut DEPTH: i32 = 0;

pub fn push() {
    unsafe {
        DEPTH += 1;
        println!("  push %rax");
    }
}

pub fn pop(str: &str) {
    unsafe {
        DEPTH -= 1;
        println!("  pop {}", str);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error(&format!("{} : invalid number of arguments", args[0]));
    }

    CURRENT_INPUT.set(args[1].clone()).unwrap();

    let token = tokenize(CURRENT_INPUT.get().unwrap().as_str());
    let (node, token) = expr(token);

    if token.kind != TokenKind::End {
        error_at(token.location, "extra token");
    }

    println!("  .global main");
    println!("main:");

    gen_expr(node);
    println!("  ret");

    assert!(unsafe { DEPTH } == 0, "stack depth mismatch");
}
