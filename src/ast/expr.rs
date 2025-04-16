use crate::ast::err::ParseError;

#[derive(Debug)]
pub struct Expr {
}

impl Expr {
    pub fn parse(mut input: &str) -> Result<(Self, &str), ParseError> {
        input = input.strip_prefix("expr").ok_or(ParseError)?.trim_ascii_start();
        Ok((Self { }, input))
    }
}