use std::collections::HashMap;

use crate::{ast::*, error::{Result, error}};

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    None,
}

pub struct Environment {
    variables: HashMap<String, Value>,
    functions: HashMap<String, FunctionDef>,
}

pub struct Evaluator {
    env: Environment
}

pub struct FunctionDef {
    body: Block,
    params: Vec<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment { 
                variables: HashMap::new(), 
                functions: HashMap::new() 
            }
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Option<Value>> {
        for stmt in program.body {
            self.eval_stmt(&stmt);
        }

        if let Some(p) = program.tail {
            return Ok(Some(self.eval_expr(p)?));
        }

        Ok(None)
    }

    fn eval_fn(&mut self, f: FunctionDef) -> Result<FunctionDef> {
        Ok(FunctionDef {
            body: f.body,
            params: f.params
        })
    }
    fn eval_binary_op(&mut self, op: BinaryOp, lhs: Value, rhs: Value, span: crate::span::Span) -> Result<Value> {
        match (op, lhs, rhs) {
            (BinaryOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (BinaryOp::Subtract, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (BinaryOp::Multiply, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (BinaryOp::Divide, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            (BinaryOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            (BinaryOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            (BinaryOp::Equal, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (BinaryOp::NotEqual, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (BinaryOp::LessThan, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (BinaryOp::LessOrEqual, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (BinaryOp::GreaterThan, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (BinaryOp::GreaterOrEqual, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(error(span, "unsupported binary operation"))

        }
    }
    fn eval_expr(&mut self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Binary(b) => {
                let lhs = self.eval_expr(b.lhs)?;
                let rhs = self.eval_expr(b.rhs)?;
                self.eval_binary_op(b.op, lhs, rhs, b.span)
            },
            Expr::Int(i) => {
                Ok(Value::Int(i.value))
            },
            Expr::Str(s) => {
                Ok(Value::Str(s.value))
            },
            Expr::If(i) => {
                i.branches.iter()
                    .find(|branch| self.eval_expr(branch.cond.clone())? == Value::Bool(true))
            },
            Expr::Identifier(i) => {
                Ok(
                    self.env
                        .variables
                        .get(&i.name)
                        .cloned()
                        .expect(format!("undefined variable: {}", i.name).as_str()))
            },
            Expr::Block(b) => {
                self.eval_block(*b)
            },
            Expr::Call(c) => {
                self.eval_fn(c)
            },
            Expr::Unary(u) => {
                self.eval_expr(e)
            }
        }
    }
    fn eval_let(&mut self, f: FunctionDef) {

    }
    fn eval_block(&mut self, block: Block) -> Result<Value> {
        for b in block.body.iter() {
            self.eval_stmt(b)?;
        }

        if let Some(tail) = block.tail {
            return self.eval_expr(tail)
        }
        
        Ok(Value::None)
    }
    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Fn(f) => {
                self.env.functions.insert(f.name, FunctionDef { body: f.body, params: f.params });
                return Ok(())
            },
            Stmt::Let(l) => {
                let val = self.eval_expr(l.value)?;
                self.env.variables.insert(l.name, val);
            },
            // Stmt::Expr(e) => {
            //     self.eval_expr(*e)?;
            //     return Ok(())
            // },
            _ => panic!("not supported")    
        }
        Ok(())
    }
}