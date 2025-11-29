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
    pub name: String,
    pub value: Expr,
}

#[derive(Debug)]
pub enum Expr {
    If(Box<ExprIf>),
    Block(Box<Block>),
    Str(Box<ExprStr>),
    Int(Box<ExprInt>),
    Identifier(Box<ExprIdent>),
    Call(Box<ExprCall>),
    Binary(Box<ExprBinary>),
    Unary(Box<ExprUnary>)
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

#[derive(Debug)]
pub struct ExprBinary {
    pub lhs: Expr,
    pub op: BinaryOp,
    pub rhs: Expr,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ExprUnary {
    pub rhs: Expr,
    pub op: UnaryOp,
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Not
}

#[derive(Debug)]
pub struct ExprCall {
    pub callee: Expr,
    pub args: Vec<Expr>
}

#[derive(Debug)]
pub struct ExprInt {
    pub value: i64
}

#[derive(Debug)]
pub struct ExprStr {
    pub value: String
}

#[derive(Debug)]
pub struct ExprIdent {
    pub name: String
}

#[derive(Debug)]
pub struct ExprBlock {
    pub inner: Block
}
#[derive(Debug)]
pub struct Block {
    pub body: Vec<Stmt>,
    pub tail: Option<Expr>
}
