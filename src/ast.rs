#[derive(Debug)]
pub struct Program {
    pub body: Vec<Stmt>,
    pub tail: Option<Expr>
}

#[derive(Debug)]
pub enum Stmt {
    Fn(Box<StmtFn>),
    Let(Box<StmtLet>),
    Expr(Box<Expr>)
}


#[derive(Debug)]
pub struct StmtFn {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block
}
#[derive(Debug)]
pub struct StmtLet {
    name: String,
    value: String,
}

#[derive(Debug)]
pub enum Expr {
    If(Box<ExprIf>)
}

#[derive(Debug)]
pub struct ExprIf {
    pub branches: Vec<IfBranch>,
    pub tail: Option<Block>
}

#[derive(Debug)]
pub struct IfBranch {
    pub cond: Expr,
    pub body: Block, 
}

pub struct ExprBinary {
    pub lhs: Expr,
    pub op: BinaryOp,
    pub rhs: Expr,
}

pub enum BinaryOp{
    Add,
    Subtract,
    Multiply,
    Divide,
    Or,
    And,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual
}

pub struct ExprUnary {
    pub rhs: Expr,
    pub op: UnaryOp,
}

pub enum UnaryOp {
    Minus,
    Not
}

pub struct ExprCall {
    pub callee: Expr,
    pub args: Vec<Expr>
}

pub struct ExprInt {
    pub value: i64
}

pub struct ExprStr {
    pub value: String
}

pub struct ExprIdent {
    pub name: String
}

pub struct ExprBlock {
    pub inner: Block
}
#[derive(Debug)]
pub struct Block {
    pub body: Vec<Stmt>,
    pub tail: Option<Expr>
}
