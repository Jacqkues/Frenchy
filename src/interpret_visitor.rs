use std::clone;
use std::collections::HashMap;
use std::rc::Rc;
use std::{ cell::RefCell};

use crate::builtin::{afficher, clock};
use crate::callable::Callable;
use crate::stmt::ReturnStmt;
use crate::token::Token;
use crate::value::{self, Function, NativeFunction};
use crate::{
    environment::Environment,
    error::RuntimeError,
    expr::{BinaryExpr, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr},
    stmt::Stmt,
    value::Value,
    visitor::{ExprVisitor, StmtVisitor},
};
#[derive(Debug,Clone)]
pub struct InterpretVisitor {
    pub global: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Token, usize>,
}

impl ExprVisitor for InterpretVisitor {
    type Output = Result<Value, RuntimeError>;

    fn visit_call_expr(&mut self, expr: &crate::expr::CallExp) -> Self::Output {
        let callee = self.evaluate(&expr.callee)?;
        
        let mut arguments = Vec::new();
        for argument in &expr.arguments {
            let arg = self.evaluate(argument)?;

            if arg != Value::Nil {
                arguments.push(arg);
               
            }
        }
      //  println!("\t\t\t\t[argument : {:?}]", &arguments);
        match callee {
            Value::NativeFunction(function) => {
                println!("call native function : {:?}", &function.name);
                if arguments.len() != function.arity {
                    return Err(RuntimeError::Error {
                        token: expr.paren.clone(),
                        message: format!(
                            "Expected {} arguments but got {}.",
                            function.arity,
                            arguments.len()
                        ),
                    });
                }
                function.call(self, arguments)
            }
            Value::Function(function) => {
                // println!("{}({:?})", &function.stmt.name, &arguments);
                if arguments.len() != function.stmt.params.len() {
                    return Err(RuntimeError::Error {
                        token: expr.paren.clone(),
                        message: format!(
                            "Expected {} arguments but got {}.",
                            function.stmt.params.len(),
                            arguments.len()
                        ),
                    });
                }
                println!("call function : {:?}", &function.stmt.name);
                function.call(self, arguments)
            }
            _ => Err(RuntimeError::Error {
                token: expr.paren.clone(),
                message: "Can only call functions and classes.".to_string(),
            }),
        }
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::LogicalExpr) -> Self::Output {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.lexeme == "or" {
            if InterpretVisitor::is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !InterpretVisitor::is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&expr.right)
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::VariableExpr) -> Self::Output {
        //println!("---------------getting variable : {:?}", &expr.name.lexeme);
       // let ret = self.environment.borrow_mut().get(&expr.name).unwrap();
       /*  println!(
            "\tlecture de la variable : {:?} : {:?}",
            &expr.name.lexeme, &ret
        );*/

        Ok(self.lookup_variable(&expr.name).unwrap())
    }

    fn visit_assign_var_expr(&mut self, expr: &crate::expr::AssignVarExpr) -> Self::Output {
       /*  let val = self.evaluate(&expr.value)?;
        println!(
            "---------------assigning variable {:?} : {:?}",
            &expr.name, &val
        );
        self.environment
            .borrow_mut()
            .assign(&expr.name, val.clone())?;
        Ok(val)*/

        let val = self.evaluate(&expr.value)?;

        if let Some(distance) = self.locals.get(&expr.name) {
            self.environment.borrow_mut().assign_at(*distance, &expr.name, val.clone())?;
        } else {
            self.global.borrow_mut().assign(&expr.name, val.clone())?;
        }

        Ok(val)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Value, RuntimeError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;
       // println!("\t [operation : {}],", &expr);
        match expr.operator.lexeme.as_str() {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            "*" => Ok(left * right),
            "/" => Ok(left / right),
            // "%" => Ok(left % right),
            ">" => Ok(Value::Boolean(left > right)),
            "<" => Ok(Value::Boolean(left < right)),
            ">=" => Ok(Value::Boolean(left >= right)),
            "<=" => Ok(Value::Boolean(left <= right)),
            "==" => Ok(Value::Boolean(left == right)),
            "!=" => Ok(Value::Boolean(left != right)),
            _ => Err(RuntimeError::Error {
                token: expr.operator.clone(),
                message: "Unknown operator".to_string(),
            }),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Value, RuntimeError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Value, RuntimeError> {
        match &expr.value {
            Literal::Number(token) => Ok(Value::Number(token.lexeme.parse::<f64>().unwrap())),
            Literal::String(token) => Ok(Value::String(token.lexeme.clone())),
            Literal::Boolean(token) => Ok(Value::Boolean(token.lexeme.parse::<bool>().unwrap())),
            Literal::Nil => Ok(Value::Nil),
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Value, RuntimeError> {
        let right = match self.evaluate(&expr.right) {
            Ok(Value::Number(num)) => num,
            Ok(Value::Boolean(bool)) => bool as i32 as f64,
            _ => Err(RuntimeError::Error {
                token: expr.operator.clone(),
                message: "Operand must be a number or boolean".to_string(),
            })?,
        };
        match expr.operator.lexeme.as_str() {
            "-" => Ok(Value::Number(-right)),
            "!" => Ok(Value::Boolean(!InterpretVisitor::make_bool_value(right))),
            _ => Err(RuntimeError::Error {
                token: expr.operator.clone(),
                message: "Unknown operator".to_string(),
            })?,
        }
    }
}

impl StmtVisitor for InterpretVisitor {
    type Output = Result<(), RuntimeError>;

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Output {
        match stmt.value {
            Some(ref expr) => {
                let value = self.evaluate(expr)?;
                println!("[return value : {}]", &value);
                Err(RuntimeError::Return(value))
            }
            None => Err(RuntimeError::Return(Value::Nil)),
        }
    }

    fn visit_function_stmt(&mut self, stmt: &crate::stmt::FunctionStmt) -> Self::Output {
        let function = Value::Function(Function {
            stmt: stmt.clone(),
            closure: Rc::clone(&self.environment),
        });
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), function);
        Ok(())
    }
    fn visit_while_stmt(&mut self, stmt: &crate::stmt::WhileStmt) -> Self::Output {
        println!("while condition : {:?}", &stmt.condition);
        println!();
        println!("while body : {:?} ", &stmt.body);
        while InterpretVisitor::is_truthy(&self.evaluate(&stmt.condition).unwrap()) {
            self.execute(&stmt.body);
        }
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &crate::stmt::IfStmt) -> Self::Output {
        let condition = self.evaluate(&stmt.condition)?;

        //println!("\t\t[condition:  {} resultat : {:?}]",&stmt.condition, &condition);
        if InterpretVisitor::is_truthy(&condition) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::ExpressionStmt) -> Self::Output {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &crate::stmt::BlockStmt) -> Self::Output {
        //  let previous_environment = Rc::clone(&self.environment);

        // Create a new environment for the block
        /*   self.environment = Rc::new(RefCell::new(Environment::new_enclosed(
            self.environment.clone(),
        )));*/

        self.execute_block(
            &stmt.statements,
            Rc::new(RefCell::new(Environment::new_enclosed(
                &self.environment,
                4,
            ))),
        )?;

        // Restore the previous environment after executing the block
        //  self.environment = previous_environment;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::PrintStmt) -> Self::Output {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", self.stringify(&value));

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::VarStmt) -> Self::Output {
        let mut val = None;
        match &stmt.initializer {
            Some(ref expr) => {
                val = Some(self.evaluate(expr)?);
            }
            None => val = Some(Value::Nil),
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), val.unwrap());

        Ok(())
    }
}

impl InterpretVisitor {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(Environment::new()));
        /*  let env = InterpretVisitor {
            global: Rc::new(RefCell::new(Environment::new())),
        };*/

        let clock_function = NativeFunction {
            arity: 0,
            name: "clock".to_string(),
            function: clock,
        };

        let print_function = NativeFunction {
            arity: 1,
            name: "print".to_string(),
            function: afficher,
        };

        let clock_value = Value::NativeFunction(clock_function);
        let print_value = Value::NativeFunction(print_function);
        global.borrow_mut().define("clock".to_string(), clock_value);

        global.borrow_mut().define("print".to_string(), print_value);

        InterpretVisitor {
            global: Rc::clone(&global),
            environment: Rc::clone(&global),
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            stmt.accept(self)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }
    fn make_bool_value(val: f64) -> bool {
        val == 1.0
    }

    fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Nil => "nil".to_string(),
            Value::Number(num) => num.to_string(),
            Value::String(string) => string.clone().to_string(),
            Value::Boolean(bool) => bool.to_string(),
            Value::NativeFunction(function) => format!("{:?}", function),
            Value::Function(function) => format!("{:?}", function),
        }
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), RuntimeError> {
        

        let previous = self.environment.clone();

        self.environment = environment;

        self.interpret(stmts)?;

        self.environment = previous;

        Ok(())
    }

    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Number(num) => *num != 0.0,
            Value::String(string) => !string.is_empty(),
            Value::Boolean(bool) => *bool,
            Value::NativeFunction(_) => true,
            Value::Function(_) => true,
        }
    }

    pub fn resolve(&mut self, name: &Token, depth: usize) {
        println!("inserting variable : {} at {} ", &name.lexeme, depth);
        self.locals.insert(name.clone(), depth );
    }

    fn lookup_variable(&self, name: &Token) -> Result<Value, RuntimeError> {

        if let Some(distance) = self.locals.get(name) {
          //  println!("distance : {:?}",distance);
          //  println!("envitonnement : {:?}",self.environment);
          //  println!("locals : {:?}",self.locals);
            self.environment.borrow().get_at(*distance,&name.lexeme)
           
        } else {
            self.global.borrow().get(&name)
        }
    }
}
