use crate::utils::error_at;

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
    pub fn new(kind: TokenKind, location: usize) -> Self {
        Token {
            kind,
            num: None,
            string: None,
            next: None,
            location,
        }
    }

    pub fn next(&mut self) -> Option<&mut Box<Token>> {
        self.next.as_mut()
    }

    pub fn push(&mut self, token: Token) {
        let token = Box::new(token);
        if self.next.is_none() {
            self.next = Some(token);
        } else {
            let mut current = self.next().unwrap();
            while current.next.is_some() {
                current = current.next().unwrap();
            }
            current.next = Some(token);
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
}
