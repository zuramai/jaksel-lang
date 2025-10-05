use crate::lexer::{self, Token, TokenKind};


pub struct Cursor<'src> {
    pub code: &'src str,
    pub tokens: Vec<Token>,
    pub position: usize
}

impl<'src> Cursor<'src> {
    fn advance(&mut self) {
        if &self.position >= &self.tokens.len() {
            return;
        }
        self.position += 1;
    }
    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }
    fn previous(&self) -> Token {
        self.tokens[self.position - 1].clone()
    }
    fn kind(&self) -> TokenKind {
        self.current().kind
    }
    fn at(&self, kind: TokenKind) -> bool {
        self.kind() == kind
    }
    fn was(&self, token: Token) -> bool {
        self.position - 1 > 0 && self.tokens[self.position-1] == token
    }
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            return true
        }
        false
    }
    fn must(&mut self, kind: TokenKind) -> Result<Token, String> {
        let current = self.current();
        if self.eat(kind) {
            Ok(current)
        } else {
            Err(format!("expected: {:?}, found {:?}", current.kind, current.span))
        }
    }
}