use decl::GlobalDecl;
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
    
    pub declarations: Vec<GlobalDecl>,
}

impl Program {
    pub fn parse(mut input: &str) -> Result<Self, ParseError> {
        // remove leading whitespace
        input = input.trim_ascii_start();
        
        let mut declarations = Vec::new();
        
        // process the entire file
        while !input.is_empty() {
            let declaration;
            (declaration, input) = GlobalDecl::parse(input)?;
            if let GlobalDecl::Empty = declaration { continue; }
            declarations.push(declaration);
        }
        Ok(Self { declarations })
    }
}

