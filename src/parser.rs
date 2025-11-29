use crate::ast::*;
use crate::cursor::Cursor;
use crate::error::{Result, error};
use crate::lexer::TokenKind;

pub fn parse_program(c: &mut Cursor) -> Result<Program> {
    let mut body = Vec::new();
    while !c.at(crate::lexer::TokenKind::TOK_EOF) {
        body.push(parse_stmt(c)?)
    }

    let trailing_semi = c.was(TokenKind::TOK_SEMI);
    let tail = if !trailing_semi 
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
    Ok(Program { body, tail })
}

/// parse the statement based on the keyword of the cursor position
fn parse_stmt(c: &mut Cursor) -> Result<Stmt> {
    match c.kind() {
        TokenKind::KW_FN => parse_stmt_fn(c),
        TokenKind::KW_LET => parse_stmt_let(c),
        _ => parse_stmt_expr(c)
    }
}

/// read the function name, the params, and the body
fn parse_stmt_fn(c: &mut Cursor) -> Result<Stmt> {
    assert!(c.eat(TokenKind::KW_FN));
    let name = parse_identifier(c)?;
    let params = parse_param_list(c)?;
    let body = parse_block(c)?;
    Ok(Stmt::Fn(Box::new(StmtFn { name, params, body })))
}

/// parse the name as a token
fn parse_identifier(c: &mut Cursor) -> Result<String> {
    let token = c.must(TokenKind::LIT_IDENT)?;
    Ok(c.lexeme(token).to_owned())
}

fn parse_param_list(c: &mut Cursor) -> Result<Vec<String>> {
    parse_paren_list(c, parse_identifier)
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
            params.push(elem(c)?);

            if !c.eat(TokenKind::COMMA) || c.at(TokenKind::TOK_RPAREN) {
                break;
            }
        }
    }
    c.must(TokenKind::TOK_RPAREN)?;
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
    c.eat(TokenKind::KW_LET);
    let name = parse_identifier(c)?;
    c.must(TokenKind::OP_EQ)?;
    let value = parse_expr(c)?;
    c.must(TokenKind::TOK_SEMI)?;
    Ok(Stmt::Let(Box::new(StmtLet { name, value })))
}

fn parse_stmt_expr(c: &mut Cursor) -> Result<Stmt> {
    let expr = parse_expr(c)?;

    // check if the next token is semicolon
    let semi = c.eat(TokenKind::TOK_SEMI);
    if !semi && !c.at(TokenKind::TOK_EOF) && !c.at(TokenKind::TOK_RBRACE) {
        return Err(error(
            c.current().span,
            format!("expected semicolon, got {:?}", c.kind())
        ));
    }
    Ok(Stmt::Expr(Box::new(expr)))
}

fn parse_expr(c: &mut Cursor) -> Result<Expr> {
    // start with 0 binding power
    parse_expr_bp(c, 0)
}

// parse recursive
fn parse_expr_bp(c: &mut Cursor, min_bp: u8) -> Result<Expr> {
    let mut lhs = parse_primary(c)?;
    loop {
        let op_kind = c.kind();
        
        let Some(bp) = binding_power(&op_kind) else {
            break;
        };


        if bp < min_bp {
            break;
        }

        match op_kind {
            // if binary operator, parse binary operator
            TokenKind::OP_PLUS | TokenKind::OP_MINUS |
            TokenKind::OP_STAR |  TokenKind::OP_SLASH |
            TokenKind::OP_LT | TokenKind::OP_GT |
            TokenKind::OP_NEQ | TokenKind::OP_EQEQ | 
            TokenKind::OP_AND | TokenKind::OP_OR |
            TokenKind::OP_LE | TokenKind::OP_GE => {
                c.advance();
                let op: BinaryOp = op_kind.clone().into();
                let rhs = parse_expr_bp(c, bp+1)?;
                lhs = Expr::Binary(Box::new(ExprBinary { lhs, op, rhs }))
            },
            // if open parentheses, parse the parentheses content
            TokenKind::TOK_LPAREN => {
                let args = parse_arg_list(c)?;
                lhs = Expr::Call(Box::new(ExprCall { args, callee: lhs }))
            }
            _ => break

        }

    }
    Ok(lhs)
}

fn parse_arg_list(c: &mut Cursor) -> Result<Vec<Expr>> {
    let mut args = Vec::new();
    c.must(TokenKind::TOK_LPAREN)?;
    while !c.at(TokenKind::TOK_RPAREN) {
        args.push(parse_expr(c)?);
        if !c.eat(TokenKind::COMMA) {
            break;
        }
    };
    
    c.must(TokenKind::TOK_RPAREN)?;

    Ok(args)
}

fn binding_power(kind: &TokenKind) -> Option<u8> {
    Some(match kind {
        TokenKind::OP_OR => 1,
        TokenKind::OP_AND => 2,
        TokenKind::OP_EQEQ | TokenKind::OP_NEQ => 3,
        TokenKind::OP_LT | TokenKind::OP_LE => 4,
        TokenKind::OP_GT | TokenKind::OP_GE => 5,
        TokenKind::OP_PLUS | TokenKind::OP_MINUS => 6,
        TokenKind::OP_STAR | TokenKind::OP_SLASH => 6,
        TokenKind::TOK_LPAREN => 8,
        _ => return None,
    })
}

fn parse_primary(c: &mut Cursor) -> Result<Expr> {
    let next_token = c.peek();
    match c.kind() {
        TokenKind::LIT_INT => {
            let value = c.current_lexeme().parse::<i64>().unwrap();
            c.advance();
            Ok(Expr::Int(Box::new(ExprInt { value })))
        }
        TokenKind::LIT_STR => {
            let value = c.current_lexeme().trim_matches('"').to_string();
            c.advance();
            Ok(Expr::Str(Box::new(ExprStr { value })))
        },
        TokenKind::LIT_IDENT => {
            let name = parse_identifier(c)?;
            Ok(Expr::Identifier(Box::new(ExprIdent { name })))
        },
        TokenKind::TOK_LPAREN => {
            c.must(TokenKind::TOK_LPAREN);
            let expr = parse_expr(c)?;
            c.must(TokenKind::TOK_RPAREN);
            Ok(expr)
        },
        TokenKind::TOK_LBRACE => {
            let block = parse_block(c)?;
            Ok(Expr::Block(Box::new(block)))
        },
        TokenKind::KW_IF => {
            parse_expr_if(c)
        },
        // parse -x
        TokenKind::OP_MINUS | TokenKind::OP_BANG => {
            let op = if next_token.kind == TokenKind::OP_MINUS {
                UnaryOp::Minus
            } else {
                UnaryOp::Not
            };
            let op_token = c.advance();
            // unary has high binding power, bcs it's directly tied to the expression
            let rhs = parse_expr_bp(c, 7)?;
            Ok(Expr::Unary(Box::new(ExprUnary { rhs, op })))
        }
        _ => Err(error(c.current().span, format!("Unexpected token error: {}", c.current_lexeme())))
    }
}

fn parse_expr_if(c: &mut Cursor) -> Result<Expr> {
    c.must(TokenKind::KW_IF)?;
    // parse the parentheses
    let cond = parse_expr(c)?;
    let body = parse_block(c)?;
    let mut branches = vec![IfBranch { cond, body }];
    let mut tail = None;

    while c.eat(TokenKind::KW_ELSE) {
        if c.eat(TokenKind::KW_IF) {
            let cond = parse_expr(c)?;
            let body = parse_block(c)?;
            branches.push(IfBranch { cond, body })
        } else {
            tail = Some(parse_block(c)?);
            break;
        }
    }

    Ok(Expr::If(Box::new(ExprIf{branches, tail})))

}