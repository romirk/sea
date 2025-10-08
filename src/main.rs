use crate::hir::Program;
use crate::lexer::LexerContext;
use crate::parser::Parseable;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::io::Write;
use std::{env::args_os, path::PathBuf};

mod ast;
mod hir;
mod lexer;
mod parser;
mod dbg;

fn main() -> Result<(), Box<dyn Error>> {
    // The path to the source file.
    let path: PathBuf = args_os()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: sea <path>");
            std::process::exit(1)
        })
        .into();
    let out_path = path.with_extension("ast");

    let contents = read_to_string(path)?;

    let mut ctx = LexerContext::new(&contents);
    let mut lexer = ctx.start();
    let program = Program::parse(lexer.delegate()).unwrap().into();

    let out_file = File::create(out_path)?;
    let mut writer = BufWriter::new(out_file);

    write!(&mut writer, "{:#?}", program).unwrap();

    Ok(())
}
