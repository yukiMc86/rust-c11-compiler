use once_cell::sync::OnceCell;
use std::process::exit;

pub static CURRENT_INPUT: OnceCell<String> = OnceCell::new();

pub fn error(msg: &str) -> ! {
    println!("{}", msg);
    exit(1);
}

pub fn error_at(num: usize, msg: &str) -> ! {
    let input = CURRENT_INPUT.get().unwrap();
    println!("{}", input);
    print!("{:width$}^ ", "", width = num);
    println!("{}", msg);
    exit(1);
}
