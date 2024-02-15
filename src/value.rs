use std::cell::RefCell;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::Pointer;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::callable::Callable;
use crate::environment;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::interpret_visitor::InterpretVisitor;
use crate::stmt::FunctionStmt;
use crate::stmt::Stmt;

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn increment_counter() {
    GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn get_counter() -> usize {
    GLOBAL_COUNTER.load(Ordering::SeqCst)
}


#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    NativeFunction(NativeFunction),
    Function(Function),
}

#[derive(Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub name: String,
    pub function: fn(&mut InterpretVisitor, Vec<Value>) -> Result<Value, RuntimeError>,
}

#[derive(Clone)]
pub struct Function {
    pub stmt: FunctionStmt,
    pub closure: Rc<RefCell<Environment>>,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("<fn :")
            .field("name", &self.stmt.name.lexeme)
            .finish()
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.stmt.params.len()
    }

    fn call(
        &self,
        interpreter: &mut InterpretVisitor,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {


        //println!("////////////////////////////////////{:?}",&self.closure);
        let environment = Rc::new(RefCell::new(Environment::new_enclosed(&self.closure,get_counter())));
        increment_counter();

       // println!("Function call : {:?}",self.stmt.name.lexeme);
       // println!("Env id : {:?}",environment.borrow().id);
        for (param, arg) in self.stmt.params.iter().zip(arguments.iter()) {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), arg.clone());
        }
        match interpreter.execute_block(&self.stmt.body, environment){
            Ok(_) => Ok(Value::Nil),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFunction")
            .field("arity", &self.arity)
            .field("name", &self.name)
            // .field("function", &self.function) // We ignore this field because it doesn't implement Debug
            .finish()
    }
}
impl Callable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut InterpretVisitor,
        arguments: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        (self.function)(interpreter, arguments)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            (Value::String(s1), Value::String(s2)) => Value::String(s1 + &s2),
            _ => panic!("Addition not supported for these Value types"),
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n2 != 0.0 {
                    Value::Number(n1 % n2)
                } else {
                    panic!("Cannot divide by zero")
                }
            }
            _ => panic!("Modulo operation not supported for these Value types"),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
            _ => panic!("Subtraction not supported for these Value types"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::String(s1), Value::String(s2)) => s1 == s2,
            (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2),
            _ => None,
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
            _ => panic!("Multiplication not supported for these Value types"),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n2 != 0.0 {
                    Value::Number(n1 / n2)
                } else {
                    panic!("Cannot divide by zero")
                }
            }
            _ => panic!("Division not supported for these Value types"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::NativeFunction(nf) => write!(f, "<native fn {}>", nf.name),
            Value::Function(func) => write!(f, "{:?}", func),
        }
    }
}
