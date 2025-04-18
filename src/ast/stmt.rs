use crate::ast::err::ParseError;
use crate::ast::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
    Empty,
    Block(Vec<Stmt>),
    While(WhileStmt),
    DoWhile(DoWhileStmt),
    For(ForStmt),
}

impl Stmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        // empty
        if let Some(tail) = input.strip_prefix(";") {
            input = tail.trim_ascii_start();
            return Ok((Self::Empty, input));
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

        Err(ParseError)
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
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

#[derive(Debug)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: Box<Stmt>,
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

#[derive(Debug)]
pub struct DoWhileStmt {
    pub cond: Expr,
    pub body: Box<Stmt>,
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

#[derive(Debug)]
pub struct WhileCond {
    pub cond: Expr,
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

#[derive(Debug)]
pub struct ForStmt {
    pub init: Option<Expr>,
    pub cond: Option<Expr>,
    pub step: Option<Expr>,
    pub body: Box<Stmt>,
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
