use crate::{interpret_visitor::InterpretVisitor, value::Value, error::RuntimeError};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut InterpretVisitor, arguments: Vec<Value>) -> Result<Value, RuntimeError>;
}

