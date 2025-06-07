use crate::chibicch::{Token, TokenKind};
use crate::utils::{CURRENT_INPUT, error_at};

fn parse_number(s: &str) -> (i32, usize) {
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

fn starts_with(input: &[char], pat: &str, pos: usize) -> bool {
    let pat_chars: Vec<char> = pat.chars().collect();
    if input.len() < pos + pat_chars.len() {
        return false;
    }
    for (i, &c) in pat_chars.iter().enumerate() {
        if input[pos + i] != c {
            return false;
        }
    }
    true
}

// Read a punctuator token from p and returns its length.
fn read_punct(chars: &Vec<char>, pos: usize) -> (String, usize) {
    // 双字符运算符
    for op in ["==", "!=", "<=", ">="] {
        if starts_with(chars, op, pos) {
            return (op.to_string(), 2);
        }
    }

    if chars[pos].is_ascii_punctuation() {
        (chars[pos].to_string(), 1)
    } else {
        ("".to_string(), 0)
    }
}

pub fn tokenize(input: &str) -> Box<Token> {
    CURRENT_INPUT.set(input.to_string()).unwrap();

    let mut head = Token::new_token(TokenKind::Empty, 0);
    let mut current = &mut head;
    let chars: Vec<char> = input.chars().collect();
    let mut pos = 0;

    while pos < input.len() {
        if chars[pos].is_whitespace() {
            pos += 1;
            continue;
        }

        if chars[pos].is_ascii_digit() {
            current.push(Token::new_token(TokenKind::Num, pos));
            current = current.next_mut();
            let (num, dis) = parse_number(&input[pos..]);
            current.num = Some(num);
            pos += dis;
            continue;
        }

        let (punct, dis) = read_punct(&chars, pos);
        if dis > 0 {
            current.push(Token::new_token(TokenKind::Punct, pos));
            current = current.next_mut();
            current.string = Some(punct);
            pos += dis;
            continue;
        }

        error_at(pos, "invalid token");
    }

    current.push(Token::new_token(TokenKind::EOF, pos));
    head.next()
}
