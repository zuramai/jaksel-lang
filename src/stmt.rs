use crate::cursor::Cursor;
use crate::error::Result;
use crate::lexer::TokenKind;

enum Stmt {
    Fn(Box<StmtFn>),
    Let(Box<StmtLet>),
    Expr(Box<Expr>)
}

fn parse_stmt(c: &mut Cursor) -> Result<Stmt> {
    match c.kind() {
        TokenKind::KW_FN => parse_stmt_fn(c),
        TokenKind::KW_LET => parse_stmt_let(c),
        _ => parse_stmt_expr(c)
    }
}

pub struct StmtFn {
    name: String,
    params: Vec<String>,
    body: Block
}

struct Block {
    body: Vec<Stmt>,
    tail: Option<Expr>
}

fn parse_stmt_fn(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_FN));
    let name = parse_ident(c)?;
    let params = parse_param_list(c);
    let body = parse_block(c)?;
    Ok(Stmt::Fn(Box::new(StmtFn { name, params, body })))
}

fn parse_ident(c: &mut Cursor) -> Result<String> {
    let token = c.must(TokenKind::LIT_IDENT)?;
    Ok(c.lexeme(token).to_owned())
}

fn parse_param_list(c: &mut Cursor) -> Result<Vec<String>> {
    parse_paren_list(c, parse_ident)
}

/// get all identifier inside parentheses
fn parse_paren_list<F, T>(
    c: &mut Cursor, 
    mut elem: F,
) -> Result<Vec<T>>
where 
    F: FnMut(&mut Cursor) -> Result<T>,
{
    let mut params = Vec::new();
    c.must(TokenKind::TOK_LPAREN)?;
    while !c.at(TokenKind::TOK_RPAREN) {
        loop {
            params.push(parse_ident(c)?);

            if !c.eat(TokenKind::COMMA) || c.at(TokenKind::TOK_RPAREN) {
                break;
            }
        }
    }
    c.must(TokenKind::TOK_RPAREN);
    Ok(params)
}

fn parse_block(c: &Cursor) -> Result<Block> {
    c.must(TokenKind::TOK_LBRACE)?;
    
    c.must(TokenKind::TOK_RBRACE)?;
}

fn parse_stmt_let(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_LET));
    let name = parse_ident(c)?;
    let body = parse_block(c)?;
    Ok(Stmt::Fn(Box::new(StmtFn { name, params, body })))
}

fn parse_stmt_expr(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_LET));
    let name = parse_ident(c)?;
    let params = parse_param_list(c);
    let body = parse_block(c)?;
    Ok(Stmt::Fn(Box::new(StmtFn { name, params, body })))
}



pub struct StmtLet {
    name: String,
    params: Vec<String>,
    body: Block
}