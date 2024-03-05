use std::collections::HashMap;

use crate::{
    expr::Expr,
    interpret_visitor::InterpretVisitor,
    token::Token,
    visitor::{ExprVisitor, StmtVisitor},
};

pub struct ResolverVisitor<'a> {
    interpreter: &'a mut InterpretVisitor,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> ResolverVisitor<'a> {
    pub fn new(interpreter: &'a mut InterpretVisitor) -> Self {
        ResolverVisitor{
            interpreter,
            scopes: vec![],
        }
    }

    fn begin_scope(&mut self) {
        
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn resolve(&mut self, statements: &Vec<crate::stmt::Stmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &crate::stmt::Stmt) {
        stmt.accept(self);
    }

    fn declare(&mut self, name: &str) {
        let mut is_defined = false;

        if let Some(scope) = self.scopes.last_mut() {
            is_defined = scope.contains_key(name);
            scope.insert(name.to_string(), false);
        }

        if is_defined {
            panic!("Variable with this name already declared in this scope.");
        }
    }

    fn define(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), true);
        }
    }

    fn resolve_local(&mut self, name: &Token) {
        for (i,scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, stmt: &crate::stmt::FunctionStmt) {
        self.begin_scope();
        for param in stmt.params.iter() {
            self.declare(&param.lexeme);
            self.define(&param.lexeme);
        }

        self.resolve(&stmt.body);
        self.end_scope();
    }
}

impl ExprVisitor for ResolverVisitor<'_> {
    type Output = ();
    fn visit_binary_expr(&mut self, expr: &crate::expr::BinaryExpr) {
        &expr.left.accept(self);
        &expr.right.accept(self);
    }

    fn visit_call_expr(&mut self,  expr: &crate::expr::CallExp) -> Self::Output {
        &expr.callee.accept(self);

        for arg in expr.arguments.iter() {
            &arg.accept(self);
        }
    }
    fn visit_grouping_expr(&mut self, expr: &crate::expr::GroupingExpr) {
        &expr.expression.accept(self);
    }

    fn visit_literal_expr(&mut self, _expr: &crate::expr::LiteralExpr) -> Self::Output{
        ();
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::UnaryExpr) {
        &expr.right.accept(self);
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::VariableExpr) -> Self::Output {
        if let Some(scope) = self.scopes.last() {
            if let Some(declared) = scope.get(&expr.name.lexeme) {
                if !declared {
                    panic!("Cannot read local variable in its own initializer.");
                }
            }
        }

        self.resolve_local(&expr.name);
    }
    fn visit_assign_var_expr(&mut self, expr: &crate::expr::AssignVarExpr) -> Self::Output {
        &expr.value.accept(self);
        self.resolve_local(&expr.name);
    }
    fn visit_logical_expr(&mut self, expr: &crate::expr::LogicalExpr) -> Self::Output {
        &expr.left.accept(self);
        &expr.right.accept(self);
    }
}

impl StmtVisitor for ResolverVisitor<'_> {
    type Output = ();
    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::ExpressionStmt) {
        &stmt.expression.accept(self);
    }
    fn visit_print_stmt(&mut self, stmt: &crate::stmt::PrintStmt) {
        &stmt.expression.accept(self);
    }
    fn visit_var_stmt(&mut self, stmt: &crate::stmt::VarStmt) {
        self.declare(&stmt.name.lexeme);

        if let Some(ref initializer) = stmt.initializer {
            initializer.accept(self);
        }

        self.define(&stmt.name.lexeme);
    }
    fn visit_block_stmt(&mut self, stmt: &crate::stmt::BlockStmt) {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
    }
    fn visit_if_stmt(&mut self, stmt: &crate::stmt::IfStmt) {
        &stmt.condition.accept(self);
        &stmt.then_branch.accept(self);

        if let Some(ref else_branch) = stmt.else_branch {
            else_branch.accept(self);
        }
    }
    fn visit_while_stmt(&mut self, stmt: &crate::stmt::WhileStmt) {
        &stmt.condition.accept(self);
        &stmt.body.accept(self);
    }
    fn visit_function_stmt(&mut self, stmt: &crate::stmt::FunctionStmt) {
        self.declare(&stmt.name.lexeme);
        self.define(&stmt.name.lexeme);

        self.resolve_function(&stmt);
    }
    fn visit_return_stmt(&mut self, stmt: &crate::stmt::ReturnStmt) {
        if let Some(ref value) = stmt.value {
            value.accept(self);
        }
    }
}
