use crate::ast::err::ParseError;
use crate::ast::expr::Expr;

// #[derive(Debug)]
// pub enum StmtOrBlock {
//     Stmt(Stmt),
//     Block(Block),
// }
//
// impl StmtOrBlock {
//     pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
//         if input.starts_with("{") {
//             // this is a block
//             let block;
//             (block, input) = Block::parse(input)?;
//             Ok((Self::Block(block), input))
//         } else {
//             let stmt;
//             (stmt, input) = Stmt::parse(input)?;
//             Ok((Self::Stmt(stmt), input))
//         }
//     }
// }
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
            stmts.push(stmt);
        }
        Ok((Self { stmts }, input))
    }
}

#[derive(Debug)]
pub enum Stmt {
    Empty,
    Block(Vec<Stmt>),
    While(WhileStmt),
    DoWhile(DoWhileStmt),
}

impl Stmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        // empty
        if let Some(tail) = input.strip_prefix(";") {
            input = tail.trim_ascii_start();
            return Ok((Self::Empty, input));
        }

        // block
        if input.starts_with("{") {
            let block;
            (block, input) = BlockStmt::parse(input)?;
            return Ok((Self::Block(block.stmts), input));
        }

        // while
        if let Ok((stmt, tail)) = WhileStmt::parse(input) {
            input = tail.trim_ascii_start();
            return Ok((Self::While(stmt), input));
        }
        
        // do while
        if let Ok((stmt, tail)) = DoWhileStmt::parse(input) {
            input = tail.trim_ascii_start();
            return Ok((Self::DoWhile(stmt), input));
        }

        Err(ParseError)
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
        
        input = input.strip_prefix("do").ok_or(ParseError)?.trim_ascii_start();
        (body, input) = Stmt::parse(input)?;
        (cond, input) = WhileCond::parse(input)?;
        input = input.strip_prefix(";").ok_or(ParseError)?.trim_ascii_start();
        
        Ok((Self { cond: cond.cond, body: Box::new(body) }, input))
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
