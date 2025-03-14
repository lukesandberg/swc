//! This package is considered internal and should not be used by external
//! crates.
//!
//! It may updated without proper semver bump.

pub use crate::token::{TokenType, TokenValue};

pub mod jsx;
mod regexp;
mod size_hint;
mod string;
mod token;

pub struct Lexer<'a> {
    logos: logos::Lexer<'a, TokenType>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LogosError {
    #[default]
    UnknownChar,
    UnterminatedStr,
}
