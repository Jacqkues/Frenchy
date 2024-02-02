use crate::{
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Literal},
    visitor::{ ExprVisitor},
};

pub struct PrintVisitor;

impl ExprVisitor for PrintVisitor {
    type Output = ();
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]);
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::CallExp) -> Self::Output {
        unimplemented!()
    }
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) {
        self.parenthesize("group", &[&expr.expression]);
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) {

        match &expr.value{
               Literal::Number(token) => print!(" Number({}) ", token.lexeme),
               Literal::String(token) => print!(" String({}) ", token.lexeme),
               Literal::Boolean(token) => print!(" Boolean({}) ", token.lexeme),
               Literal::Nil => print!("nil"),
        }
       // self.parenthesize(&expr.value.lexeme, &[]);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right]);
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::VariableExpr) -> Self::Output {
        self.parenthesize(&expr.name.lexeme, &[]);
    }
    fn visit_assign_var_expr(&mut self, expr: &crate::expr::AssignVarExpr) -> Self::Output {
        self.parenthesize(&expr.name.lexeme, &[]);
    }
    fn visit_logical_expr(&mut self, expr: &crate::expr::LogicalExpr) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]);
    }

}

impl PrintVisitor {
    pub fn print(&mut self, expr: &Expr) {
        expr.accept(self);
        println!();
    }
    pub fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) {
        print!("({}[", name);
        for expr in exprs {
            expr.accept(self);
        }

        print!("])");
    }
}
