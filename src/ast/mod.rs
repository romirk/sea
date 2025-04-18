use decl::GlobalDecl;
mod decl;
mod err;
mod expr;
mod hir;
mod stmt;

/// Root level of the AST
///
/// A program is simply a list of declarations
#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<GlobalDecl>,
}
