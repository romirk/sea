use crate::lexer::{Lexer, ParseError};

use super::hir::*;

impl Program {
    /// Parse a [`Program`].
    pub fn parse(lexer: Lexer<'_, '_>) -> Result<Self, ParseError> {
        todo!()
    }
}
