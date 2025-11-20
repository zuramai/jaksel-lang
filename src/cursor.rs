use crate::{error::{error, Result}, lexer::{self, Token, TokenKind}};


pub struct Cursor<'src> {
    pub code: &'src str,
    pub tokens: Vec<Token>,
    pub position: usize
}

impl<'src> Cursor<'src> {
    pub fn advance(&mut self) {
        if &self.position >= &self.tokens.len() {
            return;
        }
        self.position += 1;
    }
    /// get the string in a span of a cursor position
    pub fn lexeme(&self, token: Token) -> &'src str {
        &self.code[token.span]
    }
    pub fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }
    pub fn previous(&self) -> Token {
        self.tokens[self.position - 1].clone()
    }
    pub fn kind(&self) -> TokenKind {
        self.current().kind
    }
    pub fn at(&self, kind: TokenKind) -> bool {
        self.kind() == kind
    }


    /// check current if position - 1  is `token`
    pub fn was(&self, token: TokenKind) -> bool {
        self.position - 1 > 0 && self.previous().kind == token
    }

    /// Returns `true` and advances
    /// if the current token matches `kind`
    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            return true
        }
        
        false
    }

    /// eat current token or error
    pub fn must(&mut self, kind: TokenKind) -> Result<Token> {
        let current = self.current();
        if self.eat(kind) {
            Ok(current)
        } else {
            Err(error(current.span, "expected: {:?}, found {:?}"))
        }
    }
}