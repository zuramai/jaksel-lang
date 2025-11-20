use logos::{Logos};

use crate::span::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r]+")]
pub enum TokenKind {
    #[token("fn")] KW_FN,
    #[token("if")] KW_IF,
    #[token("else")] KW_ELSE,
    #[token("let")] KW_LET,

    #[token(";")] TOK_SEMI,
    #[token("(")] TOK_LPAREN,
    #[token(")")] TOK_RPAREN,
    #[token("{")] TOK_LBRACE,
    #[token("}")] TOK_RBRACE,
    #[token("wkwk")] TOK_WKWK,
    
    #[token(",")] COMMA,
    #[token("-")] OP_MINUS,
    #[token("+")] OP_PLUS,
    #[token("*")] OP_STAR,
    #[token("/")] OP_SLASH,
    #[token("=")] OP_EQ,
    #[token("||")] OP_OR,
    #[token("&&")] OP_AND,
    #[token("==")] OP_EQEQ,
    #[token("!=")] OP_NEQ,
    #[token("<")] OP_LT,
    #[token("<=")] OP_LE,
    #[token(">")] OP_GT,
    #[token(">=")] OP_GE,
    #[token("!")] OP_BANG,

    #[regex(r"[0-9]+")]
    LIT_INT,

    #[regex(r#""([^"\\]|\\.)*""#)]
    LIT_STR,

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    LIT_IDENT,

    TOK_ERROR,
    TOK_EOF
}

pub fn lex(src: &str) -> Vec<Token> {
    TokenKind::lexer(src)
        .spanned()
        .map(|item| match item {
            (Ok(kind), span) => Token {
                kind,
                span: span.into()
            },
            (Err(()), span) => Token {
                kind: TokenKind::TOK_ERROR,
                span: span.into()
            }
        }).chain([Token{
            kind: TokenKind::TOK_EOF,
            span: Span::empty()
        }])
        .collect()
}
