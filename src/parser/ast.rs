#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Var(String),
}

#[derive(Debug)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Print(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}