use crate::lexer::{Lexer, ParseResult};

use super::hir::*;

impl Stmt {
    /// Parse a [`Stmt`].
    pub fn parse(mut lexer: Lexer<'_, '_>) -> ParseResult<Self> {
        // empty
        if lexer.symbol(";").is_ok() {
            return Ok(lexer.finish(Self::Empty));
        }

        // block
        if lexer.symbol("{").is_ok() {
            let mut stmts = Vec::new();
            while lexer.symbol("}").is_err() {
                let stmt = Stmt::parse(lexer.delegate())?.into();
                stmts.push(stmt);
            }
            return Ok(lexer.finish(Self::Block { stmts }));
        }

        // break
        if lexer.keyword("break").is_ok() {
            lexer.symbol(";")?;
            return Ok(lexer.finish(Self::Break));
        }

        // continue
        if lexer.keyword("continue").is_ok() {
            lexer.symbol(";")?;
            return Ok(lexer.finish(Self::Continue));
        }

        // return
        if lexer.keyword("return").is_ok() {
            let expr = Expr::parse(lexer.delegate()).ok().map(|e| e.into());
            lexer.symbol(";")?;
            return Ok(lexer.finish(Self::Return { expr }));
        }

        // if
        if lexer.keyword("if").is_ok() {
            lexer.symbol("(")?;
            let cond = Expr::parse(lexer.delegate())?.into();
            lexer.symbol(")")?;
            let then = Box::new(Stmt::parse(lexer.delegate())?.into());
            lexer.keyword("else")?;
            let r#else = Box::new(Stmt::parse(lexer.delegate())?.into());
            return Ok(lexer.finish(Self::If { cond, then, r#else }));
        }

        // while
        if lexer.keyword("while").is_ok() {
            lexer.symbol("(")?;
            let cond = Expr::parse(lexer.delegate())?.into();
            lexer.symbol(")")?;
            let body = Box::new(Stmt::parse(lexer.delegate())?.into());
            return Ok(lexer.finish(Self::While { cond, body }));
        }

        // do-while
        if lexer.keyword("do").is_ok() {
            let body = Box::new(Stmt::parse(lexer.delegate())?.into());
            lexer.keyword("while")?;
            lexer.symbol("(")?;
            let cond = Expr::parse(lexer.delegate())?.into();
            lexer.symbol(")")?;
            lexer.symbol(";")?;
            return Ok(lexer.finish(Self::DoWhile { body, cond }));
        }

        // for
        if lexer.keyword("for").is_ok() {
            lexer.symbol("(")?;
            let init = Expr::parse(lexer.delegate()).ok().map(|e| e.into());
            lexer.symbol(";")?;
            let cond = Expr::parse(lexer.delegate()).ok().map(|e| e.into());
            lexer.symbol(";")?;
            let rept = Expr::parse(lexer.delegate()).ok().map(|e| e.into());
            lexer.symbol(")")?;
            let body = Box::new(Stmt::parse(lexer.delegate())?.into());
            return Ok(lexer.finish(Self::For {
                init,
                cond,
                rept,
                body,
            }));
        }

        todo!()
    }
}

impl Expr {
    pub fn parse(mut lexer: Lexer<'_, '_>) -> ParseResult<Self> {
        lexer.keyword("expr")?;
        Ok(lexer.finish(Self::Debug))
    }
}
