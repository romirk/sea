use crate::ast::decl::{DeclStmt, Id};
use crate::ast::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Empty,
    Break,
    Continue,
    Goto(Id),
    Return(Option<Expr>),
    Block(Vec<Stmt>),
    While(WhileStmt),
    DoWhile(DoWhileStmt),
    For(ForStmt),
    If(IfStmt),
    Decl(DeclStmt),
}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct DoWhileStmt {
    pub cond: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct WhileCond {
    pub cond: Expr,
}

#[derive(Debug)]
pub struct ForStmt {
    pub init: Option<Expr>,
    pub cond: Option<Expr>,
    pub step: Option<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub cond: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
