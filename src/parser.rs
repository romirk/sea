use crate::lexer::{Lexer, ParseResult};

use super::hir::*;

impl Stmt {
    /// Parse a [`Stmt`].
    pub fn parse(mut lexer: Lexer<'_, '_>) -> ParseResult<Self> {
        // empty
        if lexer.symbol(";").is_ok() {
            return Ok(lexer.finish(Self::Empty));
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

        todo!()
    }
}

impl Expr {
    pub fn parse(mut lexer: Lexer<'_, '_>) -> ParseResult<Self> {
        lexer.keyword("expr")?;
        Ok(lexer.finish(Self::Debug))
    }
}
