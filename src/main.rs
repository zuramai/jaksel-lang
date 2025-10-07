use crate::{lexer::lex, parser::parse_program};

pub mod stmt;
pub mod lexer;
pub mod cursor;
pub mod parser;
pub mod error;
pub mod span;

fn main() {
    dbg!(lex("fn main() {"));
    println!("Hello, world!");
}

fn parse(code: &str) {  
    let cursor = cursor::Cursor {
        code, 
        position: 0,
        tokens: lex(code)
    };
    parse_program(cursor);
} 