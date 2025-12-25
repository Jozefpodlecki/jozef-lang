use crate::lexer::{cursor::Cursor, token::*};


pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    fn token_for_char(c: char) -> Option<Token> {
        Some(Token::Punct(match c {
            '{' => Punct::LBrace,
            '}' => Punct::RBrace,
            '(' => Punct::LParen,
            ')' => Punct::RParen,
            ';' => Punct::Semicolon,
            '=' => Punct::Equals,
            _ => return None,
        }))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            self.cursor.skip_whitespace();
            self.cursor.skip_comment();

            let c = self.cursor.peek()?;

            if c.is_ascii_digit() {
                return Some(Token::Literal(Literal::Number(self.cursor.lex_number())));
            }

            if c.is_alphabetic() {
                let ident = self.cursor.lex_identifier();
                return Some(match ident.as_str() {
                    "fn" => Token::Keyword(Keyword::Fn),
                    "let" => Token::Keyword(Keyword::Let),
                    "print" => Token::Keyword(Keyword::Print),
                    _ => Token::Identifier(ident),
                });
            }

            self.cursor.bump();
            if let Some(tok) = Lexer::token_for_char(c) {
                return Some(tok);
            }
        }
    }
}