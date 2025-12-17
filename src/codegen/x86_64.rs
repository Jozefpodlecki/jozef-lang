use super::ir::*;

pub fn emit(insts: &[IrInst]) -> Vec<u8> {
    let mut code = Vec::new();

    for inst in insts {
        match inst {
            IrInst::MovImmToReg { reg: Reg::Rax, imm } => {
                code.push(0x48);
                code.push(0xB8);
                code.extend_from_slice(&imm.to_le_bytes());
            }

            IrInst::MovImmToReg { reg: Reg::Rdx, imm } => {
                code.push(0x48);
                code.push(0xB8);
                code.extend_from_slice(&imm.to_le_bytes());
            }

            IrInst::MovImmToReg { reg: Reg::Rcx, imm } => {
                code.push(0x48);
                code.push(0xB9);
                code.extend_from_slice(&imm.to_le_bytes());
            }

            IrInst::CallExtern { .. } => {
                // placeholder: relocation will go here
                code.push(0xE8);
                code.extend_from_slice(&0i32.to_le_bytes());
            }

            IrInst::Ret => {
                code.push(0xC3);
            }
        }
    }

    code
}