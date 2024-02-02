use core::fmt;

use crate::token::Token;

use crate::visitor::ExprVisitor;

pub enum Operator {
    
}

#[derive(Debug,Clone)]
pub enum Expr{
    BinaryExpr(BinaryExpr),
    GroupingExpr(GroupingExpr),
    LiteralExpr(LiteralExpr),
    UnaryExpr(UnaryExpr),
    VariableExpr(VariableExpr),
    AssignVarExpr(AssignVarExpr),
    LogicalExpr(LogicalExpr),
    CallExpr(CallExp)
}
#[derive(Debug,Clone)]
pub struct AssignVarExpr{
    pub name: Token,
    pub value: Box<Expr>,
}
#[derive(Debug,Clone)]
pub struct LogicalExpr{
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(Debug,Clone)]
pub struct CallExp{
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}



impl LogicalExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output{
        visitor.visit_logical_expr(self)
    }
}

impl AssignVarExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_assign_var_expr(self)
    }
}

impl fmt::Display for Expr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BinaryExpr(expr) => write!(f, "{}", expr),
            Expr::GroupingExpr(expr) => write!(f, "{}", expr),
            Expr::LiteralExpr(expr) => write!(f, "{}", expr),
            Expr::UnaryExpr(expr) => write!(f, "{}", expr),
            Expr::VariableExpr(expr) => write!(f, "{}", expr),
            Expr::AssignVarExpr(expr) => write!(f, "{}", expr),
            Expr::LogicalExpr(expr) => write!(f, "{}", expr),
            Expr::CallExpr(expr) => write!(f, "{}", expr),
        }
    }

}

impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::BinaryExpr(ref expr) => expr.accept(visitor),
            Expr::GroupingExpr(ref expr) => expr.accept(visitor),
            Expr::LiteralExpr(ref expr) => expr.accept(visitor),
            Expr::UnaryExpr(ref expr) => expr.accept(visitor),
            Expr::VariableExpr(ref expr) => expr.accept(visitor),
            Expr::AssignVarExpr(ref expr) => expr.accept(visitor),
            Expr::LogicalExpr(ref expr) => expr.accept(visitor),
            Expr::CallExpr(ref expr) => expr.accept(visitor),
        }
    }
}
#[derive(Debug,Clone)]
pub struct VariableExpr{
    pub name: Token,
}

impl VariableExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_variable_expr(self)
    }
}

impl CallExp{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_call_expr(self)
    }
}


/*impl Box<Expr> {
    fn accept(&self, visitor: &mut dyn Visitor) {
        (**self).accept(visitor)
    }
}*/

#[derive(Debug,Clone)]
pub struct GroupingExpr{
    pub expression: Box<Expr>,
}

impl GroupingExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_grouping_expr(self)
    }
}
#[derive(Debug,Clone)]
pub struct LiteralExpr{
    pub value: Literal,
}
#[derive(Debug,Clone)]
pub enum Literal {
    Number(Token),
    String(Token),
    Boolean(Token),
    Nil,
}

impl LiteralExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output{
        visitor.visit_literal_expr(self)
    }
}
#[derive(Debug,Clone)]
pub struct UnaryExpr{
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output{
        visitor.visit_unary_expr(self)
    }
}

#[derive(Debug,Clone)]
pub struct BinaryExpr{
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr{
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output{
        visitor.visit_binary_expr(self)
    }
}

impl fmt::Display for BinaryExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator.lexeme, self.right)
    }
}

impl fmt::Display for GroupingExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.expression)
    }
}

impl fmt::Display for LiteralExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Literal::Number(token) => write!(f, "{}", token.lexeme),
            Literal::String(token) => write!(f, "{}", token.lexeme),
            Literal::Boolean(token) => write!(f, "{}", token.lexeme),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for UnaryExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

impl fmt::Display for VariableExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.lexeme)
    }
}

impl fmt::Display for AssignVarExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} = {})", self.name.lexeme, self.value)
    }
}

impl fmt::Display for LogicalExpr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl fmt::Display for CallExp{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}(", self.callee);
        for arg in &self.arguments {
            write!(f, "{},", arg);
        }
        write!(f, "))")
    }
}

