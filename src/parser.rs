use crate::cursor::Cursor;

pub struct Program {
}

pub fn parse_program(c: Cursor) {
    let mut body = Vec::new();
    while !c.at(crate::lexer::TokenKind::TOK_EOF) {
        body.push()
    }
}