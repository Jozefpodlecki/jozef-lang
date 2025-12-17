use crate::parser::ast::{Program, Stmt, Expr};
use super::ir::*;

pub fn lower(program: &Program) -> Vec<IrInst> {
    let mut ir = Vec::new();

    for stmt in &program.statements {
        match stmt {
            Stmt::Let { name: _, value } => {
                if let Expr::Number(n) = value {
                    ir.push(IrInst::MovImmToReg {
                        reg: Reg::Rax,
                        imm: *n,
                    });
                }
            }

            Stmt::Print(expr) => {
                if let Expr::Number(n) = expr {
                    ir.push(IrInst::MovImmToReg {
                        reg: Reg::Rcx,
                        imm: *n,
                    });
                    ir.push(IrInst::CallExtern {
                        name: "print_i64".to_string(),
                    });
                }
            }
        }
    }

    ir.push(IrInst::Ret);
    ir
}