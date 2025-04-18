use crate::ast::decl::{Decl, DeclStmt, GlobalDecl, Id, ParamList, Type, TypedefStmt};
use crate::ast::err::ParseError;
use crate::ast::expr::Expr;
use crate::ast::stmt::{BlockStmt, DoWhileStmt, ForStmt, IfStmt, Stmt, WhileCond, WhileStmt};
use crate::ast::Program;

impl Program {
    pub fn parse(mut input: &str) -> Result<Self, ParseError> {
        // remove leading whitespace
        input = input.trim_ascii_start();

        let mut declarations = Vec::new();

        // process the entire file
        while !input.is_empty() {
            let declaration;
            (declaration, input) = GlobalDecl::parse(input)?;
            if let GlobalDecl::Empty = declaration {
                continue;
            }
            declarations.push(declaration);
        }
        Ok(Self { declarations })
    }
}

impl GlobalDecl {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        // empty
        if let Some(tail) = input.strip_prefix(";") {
            input = tail.trim_ascii_start();
            return Ok((Self::Empty, input));
        }

        // decl
        if let Ok((stmt, tail)) = DeclStmt::parse(input) {
            input = tail.trim_ascii_start();
            return Ok((Self::Decl(stmt), input));
        }

        // typedef
        if let Ok((stmt, tail)) = TypedefStmt::parse(input) {
            input = tail.trim_ascii_start();
            return Ok((Self::Typedef(stmt), input));
        }

        Err(ParseError)
    }
}

impl DeclStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let (decl, body);
        (decl, input) = Decl::parse(input)?;

        // if we see a `;`, this is a declaration statement
        if let Some(tail) = input.strip_prefix(";") {
            input = tail.trim_ascii_start();
            return Ok((Self { decl, body: None }, input));
        }

        // the only other scenario is a definition, so we try to parse a block
        (body, input) = BlockStmt::parse(input)?;
        Ok((
            Self {
                decl,
                body: Some(body.stmts),
            },
            input,
        ))
    }
}

impl TypedefStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input
            .strip_prefix("typedef")
            .ok_or(ParseError)?
            .trim_ascii_start();
        let (type_, alias);
        (type_, input) = Type::parse(input)?;
        (alias, input) = Id::parse(input)?;
        Ok((Self { type_, alias }, input))
    }
}

impl Decl {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let (type_, id, param_list);
        (type_, input) = Type::parse(input)?;
        (id, input) = Id::parse(input)?;

        if input.starts_with("(") {
            (param_list, input) = ParamList::parse(input)?;
            Ok((
                Self {
                    type_,
                    id,
                    param_list: Some(param_list),
                },
                input,
            ))
        } else {
            Ok((
                Self {
                    type_,
                    id,
                    param_list: None,
                },
                input,
            ))
        }
    }
}

impl ParamList {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let mut params = Vec::new();
        input = input
            .strip_prefix("(")
            .ok_or(ParseError)?
            .trim_ascii_start();

        loop {
            if let Some(tail) = input.strip_prefix(")") {
                input = tail.trim_ascii_start();
                break;
            }

            let decl;
            (decl, input) = Decl::parse(input)?;
            params.push(decl);
        }
        Ok((Self { params }, input))
    }
}

impl Type {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        if let Some(tail) = input.strip_prefix("int") {
            input = tail.trim_ascii_start();
            return Ok((Self::Int, input));
        } else if let Some(tail) = input.strip_prefix("char") {
            input = tail.trim_ascii_start();
            return Ok((Self::Char, input));
        }
        Err(ParseError)
    }
}

impl Id {
    pub fn parse(input: &str) -> Result<(Self, &str), ParseError> {
        let mut chars = input.char_indices();

        // Check the first character.
        // An identifier contains at least one character.
        let (_, first) = chars.next().ok_or(ParseError)?;
        if !first.is_ascii_alphabetic() && first != '_' {
            // Invalid first character.
            return Err(ParseError);
        }

        // Find the first invalid character; this terminates the identifier.
        let first_invalid = chars.find(|&(_, c)| !c.is_ascii_alphanumeric() && c != '_');

        // Find the offset of this invalid character.
        // If we ran out of input, then we use the whole length of the string.
        let invalid_offset = first_invalid.map_or(input.len(), |(offset, _)| offset);

        // Split the input at this point.
        let (input, rest) = input.split_at(invalid_offset);

        // Create the new 'Id'.
        let this = Self {
            name: input.to_string(),
        };

        Ok((this, rest.trim_ascii_start()))
    }
}

impl Stmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        // empty
        if let Some(tail) = input.strip_prefix(";") {
            return Ok((Self::Empty, tail.trim_ascii_start()));
        }

        // break
        if let Some(tail) = input.strip_prefix("break") {
            input = tail
                .trim_ascii_start()
                .strip_prefix(";")
                .ok_or(ParseError)?
                .trim_ascii_start();
            return Ok((Self::Break, input));
        }

        // continue
        if let Some(tail) = input.strip_prefix("continue") {
            input = tail
                .trim_ascii_start()
                .strip_prefix(";")
                .ok_or(ParseError)?
                .trim_ascii_start();
            return Ok((Self::Continue, input));
        }

        // return
        if let Some(tail) = input.strip_prefix("return") {
            input = tail.trim_ascii_start();

            let ret;
            if !input.starts_with(";") {
                let retval;
                (retval, input) = Expr::parse(input.trim_ascii_start())?;
                ret = Some(retval);
            } else {
                ret = None;
            }
            input = input
                .strip_prefix(";")
                .ok_or(ParseError)?
                .trim_ascii_start();
            return Ok((Self::Return(ret), input));
        }

        // goto
        if let Some(tail) = input.strip_prefix("goto") {
            input = tail.trim_ascii_start();
            let label;
            (label, input) = Id::parse(input)?;
            input = input
                .strip_prefix(";")
                .ok_or(ParseError)?
                .trim_ascii_start();
            return Ok((Self::Goto(label), input));
        }

        // block
        if let Ok((stmt, input)) = BlockStmt::parse(input) {
            return Ok((Self::Block(stmt.stmts), input));
        }

        // while
        if let Ok((stmt, tail)) = WhileStmt::parse(input) {
            return Ok((Self::While(stmt), tail));
        }

        // do while
        if let Ok((stmt, tail)) = DoWhileStmt::parse(input) {
            return Ok((Self::DoWhile(stmt), tail));
        }

        // for
        if let Ok((stmt, tail)) = ForStmt::parse(input) {
            return Ok((Self::For(stmt), tail));
        }

        // if
        if let Ok((stmt, tail)) = IfStmt::parse(input) {
            return Ok((Self::If(stmt), tail));
        }

        // decl
        if let Ok((stmt, tail)) = DeclStmt::parse(input) {
            // disallow function declarations in statements
            if stmt.body.is_some() {
                return Err(ParseError);
            }
            return Ok((Self::Decl(stmt), tail));
        }

        Err(ParseError)
    }
}

impl BlockStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let mut stmts = Vec::new();
        input = input
            .strip_prefix("{")
            .ok_or(ParseError)?
            .trim_ascii_start();

        loop {
            if let Some(tail) = input.strip_prefix("}") {
                input = tail.trim_ascii_start();
                break;
            }

            let stmt;
            (stmt, input) = Stmt::parse(input)?;
            // ignore empty statements -- they have no effect on blocks
            if let Stmt::Empty = stmt {
                continue;
            }
            stmts.push(stmt);
        }
        Ok((Self { stmts }, input))
    }
}

impl WhileStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let (cond, body);
        (cond, input) = WhileCond::parse(input)?;
        (body, input) = Stmt::parse(input)?;
        Ok((
            Self {
                cond: cond.cond,
                body: Box::new(body),
            },
            input,
        ))
    }
}

impl DoWhileStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let (cond, body);

        input = input
            .strip_prefix("do")
            .ok_or(ParseError)?
            .trim_ascii_start();
        (body, input) = Stmt::parse(input)?;
        (cond, input) = WhileCond::parse(input)?;
        input = input
            .strip_prefix(";")
            .ok_or(ParseError)?
            .trim_ascii_start();

        Ok((
            Self {
                cond: cond.cond,
                body: Box::new(body),
            },
            input,
        ))
    }
}

impl WhileCond {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input
            .strip_prefix("while")
            .ok_or(ParseError)?
            .trim_ascii_start()
            .strip_prefix("(")
            .ok_or(ParseError)?
            .trim_ascii_start();

        let expr;
        (expr, input) = Expr::parse(input)?;

        input = input
            .strip_prefix(")")
            .ok_or(ParseError)?
            .trim_ascii_start();

        Ok((Self { cond: expr }, input))
    }
}

impl ForStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input
            .strip_prefix("for")
            .ok_or(ParseError)?
            .trim_ascii_start()
            .strip_prefix("(")
            .ok_or(ParseError)?
            .trim_ascii_start();

        let (mut init, mut cond, mut step) = (None, None, None);

        if !input.starts_with(";") {
            let init_expr;
            (init_expr, input) = Expr::parse(input)?;
            init = Some(init_expr);
        }

        input = input
            .strip_prefix(";")
            .ok_or(ParseError)?
            .trim_ascii_start();

        if !input.starts_with(";") {
            let cond_expr;
            (cond_expr, input) = Expr::parse(input)?;
            cond = Some(cond_expr);
        }

        input = input
            .strip_prefix(";")
            .ok_or(ParseError)?
            .trim_ascii_start();

        if !input.starts_with(")") {
            let step_expr;
            (step_expr, input) = Expr::parse(input)?;
            step = Some(step_expr);
        }

        input = input
            .strip_prefix(")")
            .ok_or(ParseError)?
            .trim_ascii_start();

        let body;
        (body, input) = Stmt::parse(input)?;

        Ok((
            Self {
                init,
                cond,
                step,
                body: Box::new(body),
            },
            input,
        ))
    }
}

impl IfStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input
            .strip_prefix("if")
            .ok_or(ParseError)?
            .trim_ascii_start()
            .strip_prefix("(")
            .ok_or(ParseError)?
            .trim_ascii_start();

        let cond;
        (cond, input) = Expr::parse(input)?;

        input = input
            .strip_prefix(")")
            .ok_or(ParseError)?
            .trim_ascii_start();

        let then_branch;
        (then_branch, input) = Stmt::parse(input)?;

        if input.starts_with("else") {
            input = input
                .strip_prefix("else")
                .ok_or(ParseError)?
                .trim_ascii_start();
            let else_branch;
            (else_branch, input) = Stmt::parse(input)?;
            return Ok((
                Self {
                    cond,
                    then_branch: Box::new(then_branch),
                    else_branch: Some(Box::new(else_branch)),
                },
                input,
            ));
        }

        Ok((
            Self {
                cond,
                then_branch: Box::new(then_branch),
                else_branch: None,
            },
            input,
        ))
    }
}

impl Expr {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input
            .strip_prefix("expr")
            .ok_or(ParseError)?
            .trim_ascii_start();
        Ok((Self {}, input))
    }
}
