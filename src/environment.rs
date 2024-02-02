use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{value::Value, token::Token, error::RuntimeError};
#[derive(Clone,Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
    pub id: usize,
}


impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
            id: 0,
        }
    }

    pub fn new_enclosed(enclosing: &Rc<RefCell<Environment>>,id: usize) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
            id: id,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            println!("Env id in enclosing: {:?}",self.id);
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::Error {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::Error {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }
}