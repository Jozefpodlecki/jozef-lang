use std::env;
use clap::Parser;
use log::*;
use anyhow::{bail, Result};

use crate::{args::CommandArgs, utils::read_source_file};

mod args;
mod utils;
mod semantic;
mod parser;
mod lexer;
mod codegen;

fn configure_logging() -> Result<()> {
    let is_debug = cfg!(debug_assertions);

    if env::var_os("RUST_LOG").is_none() {
        if is_debug {
            unsafe { env::set_var("RUST_LOG", "debug"); }
        } else {
            unsafe { env::set_var("RUST_LOG", "info"); }
        }
    }

    flexi_logger::Logger::try_with_env()?
        .log_to_stdout()
        .start()?;

    Ok(())
}

fn main() {
    configure_logging().unwrap();
    let args = CommandArgs::parse();

    let source = read_source_file(&args.input).unwrap();

    let tokens = lexer::lex(&source);
    debug!("Tokens: {:?}", tokens);

    let program = parser::parse(tokens);
    debug!("Program: {:?}", program);

    let result = semantic::analyze(&program);

    if let Err(err) = result {
        error!("{}", err);
        std::process::exit(1);
    }

    let output = args.output.unwrap_or_else(|| "output.exe".to_string());
    
    match codegen::generate(&program, &output) {
        Ok(_) => {
            debug!("Saved to {}", output);
        },
        Err(err) => {
            error!("Error: {err}");
        },
    }

}