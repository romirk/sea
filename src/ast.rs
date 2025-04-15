#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

impl Program {
    pub fn parse(mut input: &str) -> Result<(Self), ParseError> {
        let mut declarations = Vec::new();
        while !input.is_empty() {
            let declaration;
            (declaration, input) = Declaration::parse(input)?;
            declarations.push(declaration);
        }
        Ok(Self { declarations })
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub type_: Type,
    pub id: Id,
    pub body: Option<Body>,
}

impl Declaration {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let (type_, id, body);
        (type_, input) = Type::parse(input)?;
        (id, input) = Id::parse(input)?;

        if let Some(tail) = input.strip_prefix(";") {
            input = tail;
            return Ok((
                Self {
                    type_,
                    id,
                    body: None,
                },
                input,
            ));
        }

        // TODO param list for function decls

        (body, input) = Body::parse(input)?;
        Ok((
            Self {
                type_,
                id,
                body: Some(body),
            },
            input,
        ))
    }
}

#[derive(Debug)]
pub enum Type {
    Int,
    Char,
}

impl Type {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        if let Some(tail) = input.strip_prefix("int") {
            input = tail;
            return Ok((Self::Int, input));
        } else if let Some(tail) = input.strip_prefix("char") {
            input = tail;
            return Ok((Self::Char, input));
        }
        Err(ParseError)
    }
}

#[derive(Debug)]
pub struct Id {
    pub name: String,
}

impl Id {
    pub fn parse(input: &str) -> Result<(Self, &str), ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Body {
    pub stmt: CompoundStmt,
}

impl Body {
    pub fn parse(input: &str) -> Result<(Self, &str), ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct CompoundStmt {
    pub stmts: Vec<MaybeCompoundStmt>,
}

impl CompoundStmt {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum MaybeCompoundStmt {
    Stmt(Stmt),
    CompoundStmt(CompoundStmt),
}

impl MaybeCompoundStmt {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Nothing,
}

impl Stmt {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Expr {}

impl Expr {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        todo!()
    }
}

/// Parsing failed.
#[derive(Debug)]
pub struct ParseError;
