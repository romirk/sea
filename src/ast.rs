pub struct Program {
    pub declarations: Vec<Declaration>,
}

pub struct Declaration {
    pub type_: Type,
    pub id: Id,
    pub body: Option<Body>,
}

pub enum Type {
    Int,
    Char,
}

pub struct Id {
    pub name: String,
}

pub struct Body {
    pub stmt: CompoundStmt,
}

pub struct CompoundStmt {
    pub stmts: Vec<MaybeCompoundStmt>,
}

pub enum MaybeCompoundStmt {
    Stmt(Stmt),
    CompoundStmt(CompoundStmt),
}

pub enum Stmt {
    Expr(Expr),
}

pub struct Expr {}
