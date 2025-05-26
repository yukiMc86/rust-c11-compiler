mod utils;

use std::env;
use std::process::exit;
use utils::parse_number;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        exit(1);
    }

    let mut p = args[1].as_str();

    let (num, rest) = parse_number(&p);
    p = rest;

    println!("  .global main");
    println!("main:");
    println!("  mov ${}, %rax", num);

    while !p.is_empty() {
        let c = p.chars().next().unwrap();
        match c {
            '+' => {
                p = &p[1..];
                let (num, rest) = parse_number(p);
                println!("  add ${}, %rax", num);
                p = rest;
            }
            '-' => {
                p = &p[1..];
                let (num, rest) = parse_number(p);
                println!("  sub ${}, %rax", num);
                p = rest;
            }
            _ => {
                eprintln!("unexpected character: '{}'", c);
                exit(1);
            }
        }
    }

    println!("  ret");
}
