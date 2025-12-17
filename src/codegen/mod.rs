pub mod interpreter;
// pub mod llvm_codegen;
mod custom_codegen;
mod lower;
mod ir;
mod x86_64;
mod generate;

pub use generate::generate;