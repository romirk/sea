use crate::lexer::{Lexer, ParseResult};

use super::hir::*;
pub trait Parseable {
    fn parse(lexer: Lexer) -> ParseResult<Self>
    where
        Self: std::marker::Sized;
}
impl Parseable for Type {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        for (std_type, concrete) in STD_TYPES.entries() {
            if lexer.keyword(std_type).is_ok() {
                return Ok(lexer.finish(concrete.clone()));
            }
        }

        // attempt to parse user-defined type or fail
        let name = lexer.ident()?.to_string();
        Ok(lexer.finish(Self::Ident { name }))
    }
}
impl Parseable for Binding {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        if lexer.symbol("(").is_ok() {
            let inner = Box::new(Binding::parse(lexer.delegate())?.into());
            lexer.symbol(")")?;
            return Ok(lexer.finish(Self::Paren(inner)));
        }
        if lexer.symbol("*").is_ok() {
            let inner = Box::new(Binding::parse(lexer.delegate())?.into());
            return Ok(lexer.finish(Self::Pointer { inner }));
        }

        let name = lexer.ident()?.to_string();
        Ok(lexer.finish(Self::Ident { name }))
    }
}

impl Parseable for MonoDecl {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        let base = Type::parse(lexer.delegate())?.into();
        let binding = Binding::parse(lexer.delegate())?.into();
        lexer.symbol(";")?;
        Ok(lexer.finish(Self {
            attrs: (),
            base,
            binding,
        }))
    }
}

impl Parseable for Stmt {
    /// Parse a [`Stmt`].
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
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

            let r#else = if lexer.keyword("else").is_ok() {
                Some(Box::new(Stmt::parse(lexer.delegate())?.into()))
            } else {
                None
            };

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

        if let Ok(result) = MonoDecl::parse(lexer.delegate()) {
            let mono_decl = result.into();
            return Ok(lexer.finish(Self::VarDefn(VarDefn {
                attrs: (),
                base: mono_decl.base,
                bindings: vec![(mono_decl.binding, None)],
            })));
        }
        todo!()
    }
}

impl Parseable for Expr {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        lexer.keyword("expr")?;
        Ok(lexer.finish(Self::Debug))
    }
}
