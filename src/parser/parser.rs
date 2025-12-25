use super::ast::{Program, Stmt, Expr};
use crate::lexer::token::{Token, Keyword, Literal, Punct};
use std::iter::Peekable;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
    Custom(String),
}

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn expect_keyword(&mut self, kw: Keyword) -> Result<(), ParseError> {
        match self.next() {
            Some(Token::Keyword(k)) if k == kw => Ok(()),
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn expect_punct(&mut self, p: Punct) -> Result<(), ParseError> {
        match self.next() {
            Some(Token::Punct(p2)) if p2 == p => Ok(()),
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Some(Token::Keyword(Keyword::Let)) => self.parse_let(),
            Some(Token::Keyword(Keyword::Print)) => self.parse_print(),
            Some(t) => Err(ParseError::UnexpectedToken(t.clone())),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.expect_keyword(Keyword::Let)?;

        let name = if let Some(Token::Identifier(n)) = self.next() {
            n
        } else {
            return Err(ParseError::Custom("Expected identifier after 'let'".into()));
        };

        self.expect_punct(Punct::Equals)?;

        let value = self.parse_expr()?;

        self.expect_punct(Punct::Semicolon)?;

        Ok(Stmt::Let { name, value })
    }

    fn parse_print(&mut self) -> Result<Stmt, ParseError> {
        self.expect_keyword(Keyword::Print)?;
        self.expect_punct(Punct::LParen)?;

        let expr = self.parse_expr()?;

        self.expect_punct(Punct::RParen)?;
        self.expect_punct(Punct::Semicolon)?;

        Ok(Stmt::Print(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        match self.next() {
            Some(Token::Literal(Literal::Number(n))) => Ok(Expr::Number(n)),
            Some(Token::Identifier(name)) => Ok(Expr::Var(name)),
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}

pub fn parse_program<I>(tokens: I) -> Result<Program, ParseError>
where
    I: Iterator<Item = Token>,
{
    let mut parser = Parser::new(tokens);

    // Expect fn main() { ... }
    parser.expect_keyword(Keyword::Fn)?;
    match parser.next() {
        Some(Token::Identifier(name)) if name == "main" => {}
        Some(t) => return Err(ParseError::UnexpectedToken(t)),
        None => return Err(ParseError::UnexpectedEOF),
    }

    parser.expect_punct(Punct::LParen)?;
    parser.expect_punct(Punct::RParen)?;
    parser.expect_punct(Punct::LBrace)?;

    let mut statements = Vec::new();
    while let Some(t) = parser.peek() {
        if matches!(t, Token::Punct(Punct::RBrace)) {
            parser.next(); // consume
            break;
        }
        statements.push(parser.parse_stmt()?);
    }

    Ok(Program { statements })
}
