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

        let name = lexer.ident()?.to_string();
        if lexer.symbol("(").is_ok() {
            if lexer.symbol(")").is_ok() {
                return Ok(lexer.finish(Self::Fn {
                    inner: Box::new(Binding::Ident { name }),
                    params: Default::default(),
                }));
            }
            let mut params = Vec::new();
            loop {
                let inner = MonoDecl::parse(lexer.delegate())?.into();
                params.push(inner);
                if lexer.symbol(")").is_ok() {
                    break;
                }
                lexer.symbol(",")?;
            }
            return Ok(lexer.finish(Self::Fn {
                inner: Box::new(Binding::Ident { name }),
                params,
            }));
        }
        Ok(lexer.finish(Self::Ident { name }))
    }
}

impl Parseable for Program {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        let mut decls = Vec::new();
        while lexer.remaining_chars() > 0 {
            let decl = TopDefn::parse(lexer.delegate())?.into();
            decls.push(decl);
        }
        Ok(lexer.finish(Self { decls }))
    }
}
impl Parseable for TopDefn {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        if let Ok(fn_res) = FnDefn::parse(lexer.delegate()) {
            return Ok(lexer.finish(Self::Fn(fn_res.into())));
        }
        if let Ok(var_res) = VarDefn::parse(lexer.delegate()) {
            return Ok(lexer.finish(Self::Var(var_res.into())));
        }
        let typedef = TypeDefn::parse(lexer.delegate())?.into();
        Ok(lexer.finish(Self::Type(typedef)))
    }
}
impl Parseable for FnDefn {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        let return_type = Type::parse(lexer.delegate())?.into();
        let name = lexer.ident()?.to_string();
        lexer.symbol("(")?;
        let mut params = Vec::new();
        if lexer.symbol(")").is_err() {
            loop {
                let inner = MonoDecl::parse(lexer.delegate())?.into();
                params.push(inner);
                if lexer.symbol(")").is_ok() {
                    break;
                }
                lexer.symbol(",")?;
            }
        }
        let body = if lexer.symbol(";").is_ok() {
            None
        } else {
            Some(Block::parse(lexer.delegate())?.into())
        };
        Ok(lexer.finish(Self {
            attrs: (),
            return_type,
            name,
            params,
            body,
        }))
    }
}

impl Parseable for VarDefn {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        let base = Type::parse(lexer.delegate())?.into();
        let mut bindings = Vec::new();

        // TODO initialization stmts
        loop {
            let binding = Binding::parse(lexer.delegate())?.into();
            bindings.push((binding, None::<Expr>));
            if lexer.symbol(";").is_ok() {
                break;
            }
            lexer.symbol(",")?;
        }

        Ok(lexer.finish(Self {
            attrs: (),
            base,
            bindings,
        }))
    }
}

impl Parseable for TypeDefn {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        lexer.keyword("typedef")?;
        let base = Type::parse(lexer.delegate())?.into();
        let mut bindings = Vec::new();

        loop {
            let binding = Binding::parse(lexer.delegate())?.into();
            bindings.push(binding);
            if lexer.symbol(";").is_ok() {
                break;
            }
            lexer.symbol(",")?;
        }

        Ok(lexer.finish(Self {
            attrs: (),
            base,
            bindings,
        }))
    }
}
impl Parseable for Decl {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        let base = Type::parse(lexer.delegate())?.into();
        let mut bindings = Vec::new();

        loop {
            let binding = Binding::parse(lexer.delegate())?.into();
            bindings.push(binding);
            if lexer.symbol(";").is_ok() {
                break;
            }
            lexer.symbol(",")?;
        }

        Ok(lexer.finish(Self {
            attrs: (),
            base,
            bindings,
        }))
    }
}

impl Parseable for MonoDecl {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        let base = Type::parse(lexer.delegate())?.into();
        let binding = Binding::parse(lexer.delegate())?.into();
        Ok(lexer.finish(Self {
            attrs: (),
            base,
            binding,
        }))
    }
}

impl Parseable for Block {
    fn parse(mut lexer: Lexer) -> ParseResult<Self> {
        lexer.symbol("{")?;
        let mut stmts = Vec::new();
        while lexer.symbol("}").is_err() {
            let stmt = Stmt::parse(lexer.delegate())?.into();
            stmts.push(stmt);
        }
        Ok(lexer.finish(Self { stmts }))
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
        if let Ok(block_result) = Block::parse(lexer.delegate()) {
            let stmts = block_result.into().stmts;
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

        // goto
        if lexer.keyword("goto").is_ok() {
            let label = lexer.ident()?.to_string();
            lexer.symbol(";")?;
            return Ok(lexer.finish(Self::Goto(label)));
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

        if let Ok(result) = Decl::parse(lexer.delegate()) {
            let decl = result.into();
            return Ok(lexer.finish(Self::Decl(decl)));
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
