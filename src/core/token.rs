use crate::utils::{error, error_at, parse_number};

#[derive(PartialEq)]
pub enum TokenKind {
    Empty,
    Punctuators,
    Num,
    End,
}

pub struct Token {
    pub kind: TokenKind,
    pub num: Option<i32>,
    pub string: Option<String>,
    pub next: Option<Box<Token>>,
    pub location: usize,
}

impl Token {
    pub fn new_token(kind: TokenKind, location: usize) -> Box<Token> {
        Box::new(Token {
            kind,
            num: None,
            string: None,
            next: None,
            location,
        })
    }

    pub fn next(self) -> Box<Token> {
        match self.next {
            Some(token) => token,
            None => error("cannot call next on a non-empty token"),
        }
    }

    pub fn next_mut(&mut self) -> &mut Box<Token> {
        match self.next {
            Some(ref mut token) => token,
            None => error("cannot call next_mut on a non-empty token"),
        }
    }

    pub fn push(&mut self, token: Box<Token>) {
        match self.next {
            None => self.next = Some(token),
            _ => error("cannot push to a non-empty token"),
        }
    }

    pub fn get_number(&self) -> i32 {
        match self.kind {
            TokenKind::Num => self.num.unwrap(),
            _ => error_at(self.location, "expected a number"),
        }
    }

    pub fn eq_punct(&self, s: &str) -> bool {
        if self.kind != TokenKind::Punctuators {
            false
        } else if let Some(ref string) = self.string {
            string == s
        } else {
            false
        }
    }

    pub fn skip(self, str: &str) -> Box<Token> {
        if !self.eq_punct(str) {
            error_at(self.location, &format!("expected a '{}'", str));
        }
        self.next()
    }
}

pub fn tokenize(input: &str) -> Box<Token> {
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

        if chars[pos].is_ascii_punctuation() {
            current.push(Token::new_token(TokenKind::Punctuators, pos));
            current = current.next_mut();
            current.string = Some(chars[pos].to_string());
            pos += 1;
            continue;
        }

        error_at(pos, "invalid token");
    }

    current.push(Token::new_token(TokenKind::End, pos));
    head.next()
}
