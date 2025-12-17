pub mod token;
use token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => { chars.next(); } // skip whitespace
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            ';' => { tokens.push(Token::Semicolon); chars.next(); }
            '=' => { tokens.push(Token::Equals); chars.next(); }
            '0'..='9' => {
                let mut num = 0i64;
                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) {
                        num = num * 10 + d.to_digit(10).unwrap() as i64;
                        chars.next();
                    } else { break; }
                }
                tokens.push(Token::Number(num));
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else { break; }
                }
                let token = match ident.as_str() {
                    "fn" => Token::Fn,
                    "let" => Token::Let,
                    "print" => Token::Print,
                    _ => Token::Identifier(ident),
                };
                tokens.push(token);
            }
            _ => {
                // skip unknown characters for now
                chars.next();
            }
        }
    }

    tokens
}
