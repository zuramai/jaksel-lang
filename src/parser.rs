use std::intrinsics::unreachable;

use crate::ast::{Block, Stmt, StmtFn};
use crate::cursor::Cursor;
use crate::error::Result;
use crate::lexer::TokenKind;

pub fn parse_program(c: &mut Cursor) {
    let mut body = Vec::new();
    while !c.at(crate::lexer::TokenKind::TOK_EOF) {
        body.push(parse_stmt(c)?)
    }
}
fn parse_stmt(c: &mut Cursor) -> Result<Stmt> {
    match c.kind() {
        TokenKind::KW_FN => parse_stmt_fn(c),
        TokenKind::KW_LET => parse_stmt_let(c),
        _ => parse_stmt_expr(c)
    }
}

fn parse_stmt_fn(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_FN));
    let name = parse_ident(c)?;
    let params = parse_param_list(c)?;
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

fn parse_block(c: &mut Cursor) -> Result<Block> {
    c.must(TokenKind::TOK_LBRACE)?;

    let mut body = Vec::new();
    while !c.at(TokenKind::TOK_RBRACE) {
        body.push(parse_stmt(c)?);
    }
    
    let trailing_semi = c.was(TokenKind::TOK_SEMI);
    c.must(TokenKind::TOK_RBRACE)?;


    let tail =   if !trailing_semi 
        && let Some(tail) = body.last() 
        && let Stmt::Expr(_) = tail 
    {
        let tail = match body.pop().unwrap() {
            Stmt::Expr(tail) => *tail,
            _ => unreachable!()
        };
        Some(tail)
    } else {
        None
    };

    Ok(Block { body, tail })
}

fn parse_stmt_let(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_LET));
    let name = parse_ident(c)?;
    let params = parse_param_list(c)?;
    let body = parse_block(c)?;
    Ok(Stmt::Fn(Box::new(StmtFn { name, params, body })))
}

fn parse_stmt_expr(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_LET));
    let name = parse_ident(c)?;
    let params = parse_param_list(c)?;
    let body = parse_block(c)?;
    Ok(Stmt::Fn(Box::new(StmtFn { name, params, body })))
}



pub struct StmtLet {
    name: String,
    params: Vec<String>,
    body: Block
}