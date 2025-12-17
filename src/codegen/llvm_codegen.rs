use anyhow::Result;
use crate::parser::ast::{Program, Stmt, Expr};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, FileType};
use std::path::Path;
use std::ffi::CString;

pub fn generate_binary(program: &Program, output_path: &str) -> Result<()> {
    Target::initialize_all(&InitializationConfig::default());

    let context = Context::create();
    let module = context.create_module("main_module");
    let builder = context.create_builder();
    let i64_type = context.i64_type();

    // Create main function
    let fn_type = i64_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    // Map variable names to LLVM values
    let mut vars = std::collections::HashMap::new();

    // Prepare external printf declaration
    let i8ptr_type = context.i8_type().ptr_type(0);
    let printf_type = i64_type.fn_type(&[i8ptr_type.into()], true);
    let printf_func = module.add_function("printf", printf_type, None);

    // Generate code for each statement
    for stmt in &program.statements {
        match stmt {
            Stmt::Let { name, value } => {
                let val = match value {
                    Expr::Number(n) => i64_type.const_int(*n as u64, false),
                    Expr::Var(var_name) => *vars.get(var_name)
                        .expect(&format!("Undefined variable {}", var_name)),
                };
                let ptr = builder.build_alloca(i64_type, name);
                builder.build_store(ptr, val);
                vars.insert(name.clone(), val);
            }
            Stmt::Print(expr) => {
                let val = match expr {
                    Expr::Number(n) => i64_type.const_int(*n as u64, false),
                    Expr::Var(var_name) => *vars.get(var_name)
                        .expect(&format!("Undefined variable {}", var_name)),
                };

                // Create format string
                let format_str = CString::new("%ld\n").unwrap();
                let global_str = builder.build_global_string_ptr(&format_str.to_string_lossy(), "fmt");

                builder.build_call(
                    printf_func,
                    &[global_str.as_pointer_value().into(), val.into()],
                    "printf_call",
                );
            }
        }
    }

    // Return 0
    builder.build_return(Some(&i64_type.const_int(0, false)));

    // Prepare target machine and emit object file
    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple)?;
    let target_machine = target.create_target_machine(
        &target_triple,
        "x86-64",
        "",
        OptimizationLevel::Default,
        inkwell::targets::RelocMode::Default,
        inkwell::targets::CodeModel::Default,
    ).unwrap();

    let obj_path = Path::new(output_path).with_extension("o");
    target_machine.write_to_file(&module, FileType::Object, &obj_path)?;

    let status = std::process::Command::new("gcc")
        .arg(&obj_path)
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to link binary");
    }

    Ok(())
}
