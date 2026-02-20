use logos::Logos;

use crate::{ast::BinaryOp, span::Span};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r]+")]
pub enum TokenKind {
    #[token("so about")]
    KW_FN,
    #[token("kalo")]
    KW_IF,
    #[token("kalogak")]
    KW_ELSE,
    #[token("literally")]
    KW_LET,
    #[token("seriously")]
    KW_CONST,

    #[token("wkwk")]
    TOK_SEMI,
    #[token("(")]
    TOK_LPAREN,
    #[token(")")]
    TOK_RPAREN,
    #[token("{")]
    TOK_LBRACE,
    #[token("}")]
    TOK_RBRACE,

    #[token(",")]
    COMMA,
    #[token("-")]
    OP_MINUS,
    #[token("tambah")]
    OP_PLUS,
    #[token("kali")]
    OP_STAR,
    #[token("bagi")]
    OP_SLASH,
    #[token("itu")]
    OP_EQ,
    #[token("||")]
    OP_OR,
    #[token("&&")]
    OP_AND,
    #[token("sama dengan")]
    OP_EQEQ,
    #[token("gak")]
    OP_NEQ,
    #[token("lebih kecil")]
    OP_LT,
    #[token("lebih kecil sama dengan")]
    OP_LE,
    #[token("lebih gede")]
    OP_GT,
    #[token("lebih gede sama dengan")]
    OP_GE,
    #[token("bukan")]
    OP_BANG,

    #[regex(r"[0-9]+")]
    LIT_INT,

    #[regex(r#""([^"\\]|\\.)*""#)]
    LIT_STR,

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    LIT_IDENT,

    TOK_ERROR,
    TOK_EOF,
}

pub fn lex(src: &str) -> Vec<Token> {
    TokenKind::lexer(src)
        .spanned()
        .map(|item| match item {
            (Ok(kind), span) => Token {
                kind,
                span: span.into(),
            },
            (Err(()), span) => Token {
                kind: TokenKind::TOK_ERROR,
                span: span.into(),
            },
        })
        .chain([Token {
            kind: TokenKind::TOK_EOF,
            span: Span::empty(),
        }])
        .collect()
}

impl From<TokenKind> for BinaryOp {
    fn from(val: TokenKind) -> Self {
        match val {
            TokenKind::OP_PLUS => BinaryOp::Add,
            TokenKind::OP_MINUS => BinaryOp::Subtract,
            TokenKind::OP_SLASH => BinaryOp::Divide,
            TokenKind::OP_STAR => BinaryOp::Multiply,
            TokenKind::OP_OR => BinaryOp::Or,
            TokenKind::OP_AND => BinaryOp::And,
            TokenKind::OP_LT => BinaryOp::LessThan,
            TokenKind::OP_EQEQ => BinaryOp::Equal,
            TokenKind::OP_GT => BinaryOp::GreaterThan,
            TokenKind::OP_GE => BinaryOp::GreaterOrEqual,
            TokenKind::OP_LE => BinaryOp::LessOrEqual,
            TokenKind::OP_NEQ => BinaryOp::NotEqual,
            _ => unreachable!(),
        }
    }
}
