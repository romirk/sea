use decl_old::GlobalDecl;
mod decl_old;
mod err;
mod expr;
mod hir;
mod stmt;
mod decl;

/// Root level of the AST
///
/// A program is simply a list of declarations
#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<GlobalDecl>,
}
