#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Fn,
    Let,
    Print,
    Identifier(String),
    Number(i64),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Equals,
}