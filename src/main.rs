mod token;
mod utils;

use once_cell::sync::OnceCell;
use std::{env, process::exit};
use token::{Token, TokenKind};
use utils::{error_at, parse_number};

pub static CURRENT_INPUT: OnceCell<String> = OnceCell::new();

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        exit(1);
    }

    CURRENT_INPUT.set(args[1].clone()).unwrap();

    let mut token = tokenize(CURRENT_INPUT.get().unwrap().as_str());

    println!("  .global main");
    println!("main:");

    // The first token must be a number
    println!("  mov ${}, %rax", token.get_number());
    let mut token = token.next().unwrap();

    while token.kind != TokenKind::End {
        if token.eq_punct("+") {
            token = token.next().unwrap();
            println!("  add ${}, %rax", token.get_number());
            token = token.next().unwrap();
            continue;
        }

        if !token.eq_punct("-") {
            error_at(token.location, "expected '-'")
        }
        token = token.next().unwrap();

        println!("  sub ${}, %rax", token.get_number());
        token = token.next().unwrap();
    }

    println!("  ret");
}

fn tokenize(input: &str) -> Box<Token> {
    let mut head = Token::new(TokenKind::Empty, 0);
    let mut current = &mut head;
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0;

    while pos < input.len() {
        if chars[pos].is_whitespace() {
            pos += 1;
            continue;
        }

        if chars[pos].is_ascii_digit() {
            current.push(Token::new(TokenKind::Num, pos));
            current = current.next().unwrap();
            let (num, dis) = parse_number(&input[pos..]);
            current.num = Some(num);
            pos += dis;
            continue;
        }

        if chars[pos] == '+' || chars[pos] == '-' {
            current.push(Token::new(TokenKind::Punctuators, pos));
            current = current.next().unwrap();
            current.string = Some(chars[pos].to_string());
            pos += 1;
            continue;
        }

        error_at(pos, "invalid token");
    }

    current.next = Some(Box::new(Token::new(TokenKind::End, pos)));
    head.next.take().unwrap()
}
