//! Tokenizing source code.

//----------- LexerContext -----------------------------------------------------

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// Context for a [`Lexer`].
pub struct LexerContext<'src> {
    /// The complete input string.
    input: &'src str,

    /// The current offset into the input.
    offset: usize,
}

//--- Construction

impl<'src> LexerContext<'src> {
    /// Construct a new [`LexerContext`].
    pub const fn new(input: &'src str) -> Self {
        Self { input, offset: 0 }
    }
}

//--- Interaction

impl<'src> LexerContext<'src> {
    /// Create a [`Lexer`] from the start of the input.
    pub fn start(&mut self) -> Lexer<'_, 'src> {
        // Skip any initial whitespace.
        let start = self
            .input
            .find(|c: char| !c.is_ascii_whitespace())
            .unwrap_or(self.input.len());

        Lexer {
            context: self,
            start,
        }
    }

    /// Create a [`Lexer`] from the current input offset.
    const fn resume(&mut self) -> Lexer<'_, 'src> {
        let start = self.offset;
        Lexer {
            context: self,
            start,
        }
    }
}

//----------- Lexer ------------------------------------------------------------

/// An on-demand lexer.
pub struct Lexer<'ctx, 'src> {
    /// Context for the lexer.
    context: &'ctx mut LexerContext<'src>,

    /// The start point of this lexer instance.
    start: usize,
}

//--- Interaction

impl<'src, 'ctx> Lexer<'ctx, 'src> {
    /// The input waiting to be lexed.
    pub fn input(&self) -> &'src str {
        &self.context.input[self.context.offset..]
    }
    
    pub fn remaining_chars(&self) -> usize {
        self.input().len()
    }

    /// Create a new [`Lexer`] from the current offset.
    pub fn delegate(&mut self) -> Lexer<'_, 'src> {
        let start = self.context.offset;
        Lexer {
            context: &mut *self.context,
            start,
        }
    }

    /// Successfully finish using the lexer.
    pub fn finish<T>(mut self, value: T) -> ParseOutput<T> {
        // Mark the lexer as complete.
        self.start = self.context.offset;
        ParseOutput(value)
    }
}

//--- Drop

impl<'src, 'ctx> Drop for Lexer<'src, 'ctx> {
    /// Drop the lexer, rewinding if it was dropped too early.
    fn drop(&mut self) {
        self.context.offset = self.start;
    }
}

//--- Actual lexing

impl<'src, 'ctx> Lexer<'src, 'ctx> {
    /// Trim whitespace.
    ///
    /// Returns whether any whitespace was parsed.
    pub fn trim_ws(&mut self) -> bool {
        let input = self.input();
        let num_ws = input
            .find(|c: char| !c.is_ascii_whitespace())
            .unwrap_or(input.len());
        self.context.offset += num_ws;
        num_ws != 0
    }

    /// Parse a specific symbol.
    ///
    /// If the input did not start with the symbol, fail.
    pub fn symbol(&mut self, symbol: &str) -> Result<(), ParseError> {
        if self.input().starts_with(symbol) {
            self.context.offset += symbol.len();
            self.trim_ws();
            Ok(())
        } else {
            Err(ParseError {
                offset: self.context.offset,
                expected: vec![format!("'{symbol}'").into_boxed_str()],
            })
        }
    }

    /// Parse an identifier.
    ///
    /// If the input did not start with an identifier. fail.
    pub fn ident(&mut self) -> Result<&'src str, ParseError> {
        let input = self.input();
        let len = input
            .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .unwrap_or(input.len());

        if len != 0 && !input.starts_with(|c: char| c.is_ascii_digit()) {
            self.context.offset += len;
            self.trim_ws();
            Ok(&input[..len])
        } else {
            Err(ParseError {
                offset: self.context.offset,
                expected: vec!["identifier".into()],
            })
        }
    }

    /// Parse a specific keyword.
    ///
    /// If the input did not start with the keyword, fail.
    pub fn keyword(&mut self, keyword: &str) -> Result<(), ParseError> {
        if self.input().strip_prefix(keyword).is_some_and(|rest| {
            !rest.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
        }) {
            self.context.offset += keyword.len();
            self.trim_ws();
            Ok(())
        } else {
            Err(ParseError {
                offset: self.context.offset,
                expected: vec![format!("'{keyword}'").into_boxed_str()],
            })
        }
    }
}

//----------- ParseResult ------------------------------------------------------

/// The result of lexing/parsing.
pub type ParseResult<T> = Result<ParseOutput<T>, ParseError>;

/// The success output of lexing/parsing.
pub struct ParseOutput<T>(T);

//--- Conversion

impl<T> ParseOutput<T> {
    /// Extract the contained value.
    pub fn into(self) -> T {
        self.0
    }
}

//--- Deref

impl<T> Deref for ParseOutput<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ParseOutput<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for ParseOutput<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for ParseOutput<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> Borrow<T> for ParseOutput<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> BorrowMut<T> for ParseOutput<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

//----------- ParseError -------------------------------------------------------

/// A lexing/parsing failure.
#[derive(Clone, Debug)]
pub struct ParseError {
    /// The byte offset of the error.
    offset: usize,

    /// A list of tokens that were allowed to occur here.
    expected: Vec<Box<str>>,
}
