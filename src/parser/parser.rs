use super::ast::{Program, Stmt, Expr};
use crate::lexer::token::Token;

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut iter = tokens.into_iter().peekable();
    let mut statements = Vec::new();

    // Expect 'fn main() { ... }'
    match iter.next() {
        Some(Token::Fn) => {},
        _ => panic!("Expected 'fn' at start"),
    }

    match iter.next() {
        Some(Token::Identifier(ref name)) if name == "main" => {},
        _ => panic!("Expected 'main' identifier"),
    }

    match iter.next() {
        Some(Token::LParen) => {},
        _ => panic!("Expected '('"),
    }

    match iter.next() {
        Some(Token::RParen) => {},
        _ => panic!("Expected ')'"),
    }

    match iter.next() {
        Some(Token::LBrace) => {},
        _ => panic!(r"Expected '{{'"),
    }

    // Parse statements until RBrace
    while let Some(token) = iter.peek() {
        match token {
            Token::Let => {
                iter.next(); // consume 'let'
                let name = if let Some(Token::Identifier(n)) = iter.next() {
                    n
                } else { panic!("Expected identifier after 'let'"); };

                match iter.next() {
                    Some(Token::Equals) => {},
                    _ => panic!("Expected '=' after identifier"),
                }

                let value = if let Some(Token::Number(n)) = iter.next() {
                    Expr::Number(n)
                } else { panic!("Expected number after '='"); };

                match iter.next() {
                    Some(Token::Semicolon) => {},
                    _ => panic!("Expected ';' after assignment"),
                }

                statements.push(Stmt::Let { name, value });
            }
            Token::Print => {
                iter.next();
                match iter.next() {
                    Some(Token::LParen) => {},
                    _ => panic!("Expected '(' after 'print'"),
                }

                let expr = match iter.next() {
                    Some(Token::Number(n)) => Expr::Number(n),
                    Some(Token::Identifier(name)) => Expr::Var(name),
                    _ => panic!("Expected number or variable in print"),
                };

                match iter.next() {
                    Some(Token::RParen) => {},
                    _ => panic!("Expected ')' after print expression"),
                }

                match iter.next() {
                    Some(Token::Semicolon) => {},
                    _ => panic!("Expected ';' after print statement"),
                }

                statements.push(Stmt::Print(expr));
            }
            Token::RBrace => { iter.next(); break; }
            _ => panic!("Unexpected token {:?}", token),
        }
    }

    Program { statements }
}