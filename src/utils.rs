use once_cell::sync::OnceCell;
use std::process::exit;

pub static CURRENT_INPUT: OnceCell<String> = OnceCell::new();

pub fn parse_number(s: &str) -> (i32, usize) {
    let mut chars = s.chars();
    let mut len = 0;

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            len += 1;
        } else {
            break;
        }
    }

    if len == 0 {
        (0, 0)
    } else {
        (s[..len].parse().unwrap(), len)
    }
}

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
