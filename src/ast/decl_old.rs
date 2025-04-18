use crate::ast::decl::{Id, Type};
use crate::ast::stmt::Stmt;

#[derive(Debug)]
pub enum GlobalDecl {
    Decl(DeclStmt),
    Typedef(TypedefStmt),
    Empty,
}

/// AST node representing a declaration statement
///
/// This AST does not distinguish between declarations and definitions; a definition is simply a
/// declaration with `Some(body)`
#[derive(Debug)]
pub struct DeclStmt {
    pub decl: Decl,
    pub body: Option<Vec<Stmt>>,
}

#[derive(Debug)]
pub struct TypedefStmt {
    pub type_: Type,
    pub alias: Id,
}

/// AST node representing a declaration
///
/// A declaration consists of a type, identifier, and an optional paramater list.
#[derive(Debug)]
pub struct Decl {
    pub type_: Type,
    pub id: Id,
    pub param_list: Option<ParamList>,
}

#[derive(Debug)]
pub struct ParamList {
    pub params: Vec<Decl>,
}
