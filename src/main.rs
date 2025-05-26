use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        exit(1);
    }

    let num: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("{}: argument is not a valid integer", args[0]);
            exit(1);
        }
    };

    println!("  .global main");
    println!("main:");
    println!("  mov ${}, %rax", num);
    println!("  ret");
}
