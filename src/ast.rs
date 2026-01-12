#[derive(Debug)]
pub struct Program {
    pub body: Vec<Stmt>,
    pub tail: Option<Expr>
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Fn(Box<StmtFn>),
    Let(Box<StmtLet>),
    Expr(Box<Expr>)
}


#[derive(Debug, Clone)]
pub struct StmtFn {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block
}
#[derive(Debug, Clone)]
pub struct StmtLet {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ExprIf {
    pub branches: Vec<IfBranch>,
    pub tail: Option<Block>
}

#[derive(Debug, Clone)]
pub struct IfBranch {
    pub cond: Expr,
    pub body: Block, 
}

#[derive(Debug, Clone)]
pub struct ExprBinary {
    pub lhs: Expr,
    pub op: BinaryOp,
    pub rhs: Expr,
    pub span: crate::span::Span,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ExprUnary {
    pub rhs: Expr,
    pub op: UnaryOp,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
    Not
}

#[derive(Debug, Clone)]
pub struct ExprCall {
    pub callee: Expr,
    pub args: Vec<Expr>
}

#[derive(Debug, Clone)]
pub struct ExprInt {
    pub value: i64
}

#[derive(Debug, Clone)]
pub struct ExprStr {
    pub value: String
}

#[derive(Debug, Clone)]
pub struct ExprIdent {
    pub name: String
}

#[derive(Debug)]
pub struct ExprBlock {
    pub inner: Block
}
#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Stmt>,
    pub tail: Option<Expr>
}
