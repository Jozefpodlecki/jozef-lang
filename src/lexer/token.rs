#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Let,
    Print,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Punct {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Equals,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Punct(Punct),
}