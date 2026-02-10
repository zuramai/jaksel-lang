use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use crate::span::Span;
use crate::{
    ast::*,
    error::{Result, error},
};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    Function(Rc<FunctionValue>),
    None,
}

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

#[derive(Clone, Debug)]
pub struct FunctionDef {
    body: Block,
    params: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub name: String,
    pub params: Vec<String>,
    pub body: Block,
    pub closure: Environment,
}

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::Str(l), Self::Str(r)) => l == r,
            _ => false,
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Str(s) => !s.is_empty(),
            Value::None => false,
            Value::Function(_) => true,
            _ => false,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Bool(_) => "bool",
            Value::Int(_) => "int",
            Value::Str(_) => "string",
            Value::Function(_) => "function",
            Value::None => "none",
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    /// create a new Environment with a parent
    pub fn extend(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(v) = self.values.get(name) {
            return Some(v.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        None
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.borrow_mut().assign(name, value);
        }
        false
    }
}

impl Evaluator {
    pub fn new() -> Self {
        let env = Environment::new();
        Self {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Result<Value> {
        for stmt in program.body {
            self.eval_stmt(&stmt)?;
        }

        if let Some(p) = &program.tail {
            return self.eval_expr(p);
        }

        Ok(Value::None)
    }

    fn eval_binary_op(
        &mut self,
        op: &BinaryOp,
        lhs: Value,
        rhs: Value,
        span: Span,
    ) -> Result<Value> {
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
            _ => Err(error(span, "unsupported binary operation")),
        }
    }
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Int(i) => Ok(Value::Int(i.value)),
            Expr::Str(s) => Ok(Value::Str(s.value.clone())),
            Expr::Binary(b) => {
                let lhs = self.eval_expr(&b.lhs)?;
                let rhs = self.eval_expr(&b.rhs)?;
                self.eval_binary_op(&b.op, lhs, rhs, b.span.clone())
            }
            Expr::Block(b) => self.eval_block(b),
            Expr::Unary(u) => {
                let val = self.eval_expr(&u.rhs)?;

                match (&u.op, &val) {
                    (UnaryOp::Minus, Value::Int(i)) => Ok(Value::Int(-i)),
                    (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    (UnaryOp::Not, _) => Ok(Value::Bool(!val.is_truthy())),
                    _ => Err(error(
                        Span::empty(),
                        format!("cannot apply {:?} to {}", u.op, val.type_name()),
                    )),
                }
            }
            _ => Err(error(Span::empty(), format!("invalid expression"))),
        }
    }
    fn eval_block(&mut self, block: &Block) -> Result<Value> {
        for b in block.body.iter() {
            self.eval_stmt(b)?;
        }

        if let Some(tail) = &block.tail {
            return self.eval_expr(tail);
        }

        Ok(Value::None)
    }
    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Fn(f) => {
                let function = Value::Function(Rc::new(FunctionValue {
                    name: f.name.clone(),
                    params: f.params.clone(),
                    body: f.body.clone(),
                    closure: self.env.borrow().clone(),
                }));
                self.env.borrow_mut().assign(&f.name, function);
                return Ok(());
            }
            Stmt::Let(l) => {
                let val = self.eval_expr(&l.value)?;
                self.env.borrow_mut().define(l.name.clone(), val);
            }
            // Stmt::Expr(e) => {
            //     self.eval_expr(*e)?;
            //     return Ok(())
            // },
            _ => panic!("not supported"),
        }
        Ok(())
    }
}
