//! This package is considered internal and should not be used by external
//! crates.
//!
//! It may updated without proper semver bump.

use std::mem::take;

use swc_common::{BytePos, Span};

pub use crate::token::{Token, TokenType, TokenValue};

pub mod jsx;
mod regexp;
mod string;
mod token;

pub struct Lexer<'a> {
    logos: logos::Lexer<'a, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            logos: logos::Lexer::new(input),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LogosError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.logos.extras.had_line_break = false;
        let tt = self.logos.next()?;
        let tt = match tt {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };

        let span = self.logos.span();
        let value = take(&mut self.logos.extras.value);
        let had_line_break = self.logos.extras.had_line_break;

        Some(Ok(Token::new(
            tt,
            Span::new(BytePos(span.start as u32), BytePos(span.end as u32)),
            had_line_break,
            value,
        )))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LogosError {
    #[default]
    UnknownChar,
    UnterminatedStr,
}
