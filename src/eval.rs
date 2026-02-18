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
            Expr::If(expr_if) => {
                for branch in &expr_if.branches {
                    if self.eval_expr(&branch.cond)?.is_truthy() {
                        return self.eval_block(&branch.body);
                    }
                }
                // eval else block
                if let Some(else_block) = &expr_if.tail {
                    return self.eval_block(else_block);
                }
                return Ok(Value::None);
            }
            Expr::Identifier(ident) => match self.env.borrow().get(&ident.name) {
                Some(v) => Ok(v),
                None => Err(error(
                    Span::empty(),
                    format!("undefined variable: {}", ident.name),
                )),
            },
            Expr::Call(call) => {
                let callee = self.eval_expr(&call.callee)?;

                let args = call
                    .args
                    .iter()
                    .map(|a| self.eval_expr(a))
                    .collect::<Result<Vec<Value>>>()?;

                self.call_function(callee, args)
            }

            _ => Err(error(Span::empty(), format!("invalid expression"))),
        }
    }
    fn call_function(&mut self, callee: Value, args: Vec<Value>) -> Result<Value> {
        match callee {
            Value::Function(func) => {
                if args.len() != func.params.len() {
                    return Err(error(
                        Span::empty(),
                        format!(
                            "expected {} arguments, but got {}",
                            func.params.len(),
                            args.len()
                        ),
                    ));
                }

                let mut func_env = Environment::extend(Rc::new(RefCell::new(func.closure.clone())));

                for (param, arg) in func.params.iter().zip(args) {
                    func_env.define(param.clone(), arg);
                }

                let outer_env = Rc::clone(&self.env);
                self.env = Rc::new(RefCell::new(func_env));

                let result = self.eval_block(&func.body)?;

                self.env = outer_env;

                Ok(result)
            }
            _ => Err(error(
                Span::empty(),
                format!("{} is not callable", callee.type_name()),
            )),
        }
    }
    fn eval_block(&mut self, block: &Block) -> Result<Value> {
        let outer_env = Rc::clone(&self.env);
        let inner_env = Environment::extend(Rc::clone(&outer_env));
        self.env = Rc::new(RefCell::new(inner_env));

        for b in &block.body {
            self.eval_stmt(b)?;
        }

        let result = if let Some(tail) = &block.tail {
            self.eval_expr(tail)?
        } else {
            Value::None
        };

        self.env = outer_env;

        Ok(result)
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
                Ok(())
            }
            Stmt::Let(l) => {
                let val = self.eval_expr(&l.value)?;
                self.env.borrow_mut().define(l.name.clone(), val);
                Ok(())
            }
            Stmt::Expr(e) => {
                self.eval_expr(e)?; // do nothing and look for potential errors
                Ok(())
            }
            _ => panic!("not supported"),
        }
    }
}
