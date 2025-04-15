#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<DeclStmt>,
}

impl Program {
    pub fn parse(mut input: &str) -> Result<Self, ParseError> {
        input = input.trim_ascii_start();
        let mut declarations = Vec::new();
        while !input.is_empty() {
            let declaration;
            (declaration, input) = DeclStmt::parse(input)?;
            declarations.push(declaration);
        }
        Ok(Self { declarations })
    }
}

#[derive(Debug)]
pub struct DeclStmt {
    // pub type_: Type,
    // pub id: Id,
    // pub body: Option<CompoundStmt>,
    pub decl: Decl,
    pub body: Option<Block>,
}

impl DeclStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let (decl, body);
        (decl, input) = Decl::parse(input)?;

        if let Some(tail) = input.strip_prefix(";") {
            input = tail.trim_ascii_start();
            return Ok((Self { decl, body: None }, input));
        }

        (body, input) = Block::parse(input)?;
        Ok((
            Self {
                decl,
                body: Some(body),
            },
            input,
        ))
    }
}

#[derive(Debug)]
pub struct Decl {
    pub type_: Type,
    pub id: Id,
    pub param_list: Option<ParamList>,
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

#[derive(Debug)]
pub struct ParamList {
    params: Vec<Decl>,
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

#[derive(Debug)]
pub enum Type {
    Int,
    Char,
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

#[derive(Debug)]
pub struct Id {
    pub name: String,
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

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<MaybeCompoundStmt>,
}

impl Block {
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
            (stmt, input) = MaybeCompoundStmt::parse(input)?;
            stmts.push(stmt);
        }
        Ok((Self { stmts }, input))
    }
}

#[derive(Debug)]
pub enum MaybeCompoundStmt {
    Stmt(Stmt),
    CompoundStmt(Block),
}

impl MaybeCompoundStmt {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input
            .strip_prefix(";")
            .ok_or(ParseError)?
            .trim_ascii_start();
        Ok((Self::Stmt(Stmt::Expr(Expr {})), input))
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
