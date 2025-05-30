use std::error::Error;
use std::{env::args_os, path::PathBuf};
use hir::Stmt;
use crate::lexer::{Lexer, LexerContext};

mod ast;
mod hir;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    // The path to the source file.
    let path: PathBuf = args_os()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: sea <path>");
            std::process::exit(1)
        })
        .into();

    println!("Reading from {}", path.display());

    let contents = std::fs::read_to_string(path)?;
    let program = ast::Program::parse(&contents).unwrap_or_else(|err| {
        eprintln!("Failed to parse: {err:?}");
        std::process::exit(2)
    });
    println!("Parsed to: {program:#?}");
    
    let mut ctx = LexerContext::new(&contents);
    let lexer = ctx.start();
    
    Stmt::parse(lexer).unwrap();

    Ok(())
}
