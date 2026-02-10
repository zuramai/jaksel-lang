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

impl Into<BinaryOp> for TokenKind {
    fn into(self) -> BinaryOp {
        match self {
            Self::OP_PLUS => BinaryOp::Add,
            Self::OP_MINUS => BinaryOp::Subtract,
            Self::OP_SLASH => BinaryOp::Divide,
            Self::OP_STAR => BinaryOp::Multiply,
            Self::OP_OR => BinaryOp::Or,
            Self::OP_AND => BinaryOp::And,
            Self::OP_LT => BinaryOp::LessThan,
            Self::OP_EQEQ => BinaryOp::Equal,
            Self::OP_GT => BinaryOp::GreaterThan,
            Self::OP_GE => BinaryOp::GreaterOrEqual,
            Self::OP_LE => BinaryOp::LessOrEqual,
            Self::OP_NEQ => BinaryOp::NotEqual,
            _ => unreachable!(),
        }
    }
}
