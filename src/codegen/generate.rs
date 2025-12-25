use crate::{codegen::{custom_codegen, llvm_codegen}, parser::ast::Program};
use anyhow::Result;

pub fn generate(program: &Program, output_path: &str) -> Result<()> {
    // custom_codegen::generate_binary(program, output_path)?;
    llvm_codegen::generate_binary(program, output_path);

    Ok(())
}