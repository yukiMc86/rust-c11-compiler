mod codegen;
mod core;
mod parse;
mod tokenize;
mod utils;

use std::env;

use codegen::codegen;
use parse::parse;
use tokenize::tokenize;
use utils::error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error(&format!("{} : invalid number of arguments", args[0]));
    }

    let token = tokenize(args[1].as_str());
    let node = parse(token);
    codegen(node);
}
