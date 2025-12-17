#[derive(Debug)]
pub enum IrInst {
    MovImmToReg { reg: Reg, imm: i64 },
    CallExtern { name: String },
    Ret,
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    Rax,
    Rcx,
    Rdx,
}