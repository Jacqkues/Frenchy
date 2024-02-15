use std::{ cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::RuntimeError,
    token::{Token, TokenType},
    value::Value,
};
#[derive(Clone, Debug)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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

    pub fn new_enclosed(enclosing: &Rc<RefCell<Environment>>, id: usize) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
            id: id,
        }
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        // Get first ancestor
        let parent = self
            .enclosing
            .clone()
            .expect(&format!("No enclosing environment at {}", 1));
        let mut environment = Rc::clone(&parent);
       // println!("first ancestor: {:?}", environment);
        // Get next ancestors
        for i in 1..distance {
            let parent = environment
                .borrow_mut()
                .enclosing
                .clone()
                .expect(&format!("No enclosing environment at {}", i));
            environment = Rc::clone(&parent);
        }
        println!("\n\tancestor: {:?}\n", environment);
        environment
    }

    pub fn assign_at(&mut self, distance: i32, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if distance > 0 {
            let ancestor = self.ancestor(distance as usize);
            ancestor.borrow_mut().values.insert(name.lexeme.clone(), value);
        }
        //self.values.insert(name.lexeme.clone(), value);
        Ok(())
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Value, RuntimeError> {
        if distance > 0 {
            Ok(self
                .ancestor(distance)
                .borrow()
                .values
                .get(name)
                .expect(&format!("Undefined variable '{}'", name))
                .clone())
        } else {
            Ok(self
                .values
                .get(name)
                .expect(&format!("Undefined variable '{}'", name))
                .clone())
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
            println!("Env id in enclosing: {:?}", self.id);
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
