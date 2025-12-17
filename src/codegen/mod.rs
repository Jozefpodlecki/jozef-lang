pub mod interpreter;
pub mod llvm_codegen;

use crate::parser::ast::Program;

pub fn generate(program: &Program, output: &str) {
    // #[cfg(feature = "interpret")] 
    // interpreter::execute(program);

    llvm_codegen::generate_binary(program, output);
}