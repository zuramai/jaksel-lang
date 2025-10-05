use crate::{lexer::lex, program::parse_program};

pub mod lexer;
pub mod cursor;
pub mod program;

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