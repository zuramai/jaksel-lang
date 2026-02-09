use crate::error::Result;
use crate::{ast::Program, lexer::lex, parser::parse_program};

pub mod ast;
pub mod cursor;
pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod span;

#[cfg(test)]
pub mod tests;

fn parse(code: &str) -> Result<Program> {
    let mut cursor = cursor::Cursor {
        code,
        position: 0,
        tokens: lex(code),
    };
    parse_program(&mut cursor)
}
