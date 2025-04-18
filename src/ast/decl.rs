use crate::ast::err::ParseError;
use crate::ast::expr::Expr;

#[derive(Debug)]
pub struct Id {
    pub name: String,
}

#[derive(Debug)]
pub enum Binding {
    Anonymous,
    Id(Id),
    Pointer(Box<Self>),
    Array(Box<Self>, Expr),
    Function(Box<Self>, ParamList),
}

impl Binding {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        if let Some(tail) = input.strip_prefix("*") {
            input = tail.trim_ascii_start();
            let binding;
            (binding, input) = Binding::parse(input)?;
            return Ok((Self::Pointer(Box::new(binding)), input));
        }

        todo!()
    }
}

#[derive(Debug)]
pub struct ParamList {
    pub params: Vec<Param>,
}

#[derive(Debug)]
pub struct Param {
    pub type_: Type,
    pub binding: Binding,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Char,
}

#[derive(Debug)]
pub enum Modifier {
    Static,
    Inline,
    Const,
}

impl Modifier {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        if let Some(tail) = input.strip_prefix("const") {
            return Ok((Modifier::Const, tail.trim_ascii_start()));
        }
        if let Some(tail) = input.strip_prefix("static") {
            return Ok((Modifier::Static, tail.trim_ascii_start()));
        }
        if let Some(tail) = input.strip_prefix("inline") {
            return Ok((Modifier::Inline, tail.trim_ascii_start()));
        }
        Err(ParseError)
    }
}

#[derive(Debug)]
pub struct Decl {
    pub modifiers: Vec<Modifier>,
    pub type_: Type,
    pub bindings: Vec<Binding>,
}

impl Decl {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        let mut modifiers = Vec::new();
        while let Ok((modifier, tail)) = Modifier::parse(input) {
            input = tail;
            modifiers.push(modifier);
        }

        let type_;
        (type_, input) = Type::parse(input)?;

        let mut bindings = Vec::new();
        loop {
            let binding;
            (binding, input) = Binding::parse(input)?;
            bindings.push(binding);

            if let Some(tail) = input.strip_prefix(",") {
                input = tail.trim_ascii_start();
                continue;
            }
            break;
        }
        Ok((
            Self {
                modifiers,
                type_,
                bindings,
            },
            input,
        ))
    }
}
