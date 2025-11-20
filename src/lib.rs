use crate::{ast::Program, lexer::lex, parser::parse_program};
use crate::error::Result;

pub mod ast;
pub mod lexer;
pub mod cursor;
pub mod parser;
pub mod error;
pub mod span;

#[cfg(test)]
pub mod tests;


fn parse(code: &str) -> Result<Program> {  
    let mut cursor = cursor::Cursor {
        code, 
        position: 0,
        tokens: lex(code)
    };
    parse_program(&mut cursor)
} 