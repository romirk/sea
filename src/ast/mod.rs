use decl::DeclStmt;
use err::ParseError;

mod decl;
mod stmt;
mod err;
mod expr;

/// Root level of the AST
/// 
/// A program is simply a list of declarations
#[derive(Debug)]
pub struct Program {
    
    pub declarations: Vec<DeclStmt>,
}

impl Program {
    pub fn parse(mut input: &str) -> Result<Self, ParseError> {
        // remove leading whitespace
        input = input.trim_ascii_start();
        
        let mut declarations = Vec::new();
        
        // process the entire file
        while !input.is_empty() {
            let declaration;
            (declaration, input) = DeclStmt::parse(input)?;
            declarations.push(declaration);
        }
        Ok(Self { declarations })
    }
}

