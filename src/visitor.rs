use crate::{expr::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr, VariableExpr}, stmt::{Stmt, ExpressionStmt, PrintStmt}};



pub trait ExprVisitor {
    type Output;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Output;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Self::Output;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Self::Output;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Self::Output;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Self::Output;
    fn visit_assign_var_expr(&mut self, expr: &crate::expr::AssignVarExpr) -> Self::Output;
    fn visit_logical_expr(&mut self, expr: &crate::expr::LogicalExpr) -> Self::Output;
    fn visit_call_expr(&mut self, expr: &crate::expr::CallExp) -> Self::Output;
}

pub trait StmtVisitor {
    type Output;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Self::Output;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Output;
    fn visit_var_stmt(&mut self, stmt: &crate::stmt::VarStmt) -> Self::Output;
    fn visit_block_stmt(&mut self, stmt: &crate::stmt::BlockStmt) -> Self::Output;
    fn visit_if_stmt(&mut self, stmt: &crate::stmt::IfStmt) -> Self::Output;
    fn visit_while_stmt(&mut self, stmt: &crate::stmt::WhileStmt) -> Self::Output;
    fn visit_function_stmt(&mut self, stmt: &crate::stmt::FunctionStmt) -> Self::Output;
    fn visit_return_stmt(&mut self, stmt: &crate::stmt::ReturnStmt) -> Self::Output;
}