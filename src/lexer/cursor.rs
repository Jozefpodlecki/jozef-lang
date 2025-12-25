use std::str::Chars;

pub struct Cursor<'a> {
    input: &'a str,
    chars: Chars<'a>,
    prev: Option<char>,
    pub line: usize,
    pub column: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
            prev: None,
            line: 1,
            column: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = Some(c);

        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(c)
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    pub fn skip_comment(&mut self) {
        if self.peek() == Some('/') {
            let mut clone = self.chars.clone();
            clone.next();
            if clone.next() == Some('/') {
                // consume comment
                self.bump(); // first '/'
                self.bump(); // second '/'
                while let Some(c) = self.bump() {
                    if c == '\n' { break; }
                }
            }
        }
    }

    pub fn lex_number(&mut self) -> i64 {
        let mut num = 0;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap() as i64;
                self.bump();
            } else {
                break;
            }
        }
        num
    }

    pub fn lex_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.bump();
            } else {
                break;
            }
        }
        ident
    }
}
