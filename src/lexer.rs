use logos::{Logos};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: u32,
    pub end: u32
}

impl Span {
    #[inline]
    pub fn empty() -> Self {
        Self {
            start: 0,
            end: 0
        }
    }

    #[inline]
    pub fn start(self) -> usize {
        return self.start as usize;
    }

    #[inline]
    pub fn end(self) -> usize {
        return self.end as usize;
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start as u32,
            end: value.end as u32,
        }
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(value: Span) -> Self {
        Self {
            start: value.start as usize,
            end: value.end as usize,
        }
    }
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
