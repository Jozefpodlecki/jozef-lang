use crate::parser::ast::{Program, Stmt, Expr};
use std::collections::HashSet;
use anyhow::{Result, bail};

pub fn analyze(program: &Program) -> Result<()> {
    let mut declared = HashSet::new();

    for (idx, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Stmt::Let { name, .. } => {
                if declared.contains(name) {
                    bail!("Semantic error at statement {}: Variable '{}' already declared", idx + 1, name);
                }
                declared.insert(name.clone());
            }
            Stmt::Print(expr) => {
                if let Expr::Var(name) = expr {
                    if !declared.contains(name) {
                        bail!("Semantic error at statement {}: Variable '{}' used before declaration", idx + 1, name);
                    }
                }
            }
        }
    }

    Ok(())
}
