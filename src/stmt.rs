use crate::{expr::Expr, visitor::StmtVisitor, token::Token};


#[derive(Debug,Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
    Block(BlockStmt),
    If(IfStmt),
    While(WhileStmt),
    Function(FunctionStmt),
    Return(ReturnStmt),
}

#[derive(Debug,Clone)]
pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Option<Expr>,
}

#[derive(Debug,Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug,Clone)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug,Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
#[derive(Debug,Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}
#[derive(Debug,Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}


#[derive(Debug,Clone)]
pub struct ExpressionStmt {
    pub expression: Expr,
}
#[derive(Debug,Clone)]
pub struct PrintStmt {
    pub expression: Expr,
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Stmt::Expression(ref stmt) => stmt.accept(visitor),
            Stmt::Print(ref stmt) => stmt.accept(visitor),
            Stmt::Var(ref stmt) => stmt.accept(visitor),
            Stmt::Block(ref stmt) => stmt.accept(visitor),
            Stmt::If(ref stmt) => stmt.accept(visitor),
            Stmt::While(ref stmt) => stmt.accept(visitor),
            Stmt::Function(ref stmt) => stmt.accept(visitor),
            Stmt::Return(ref stmt) => stmt.accept(visitor),
        }
    }
}


impl ReturnStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_return_stmt(self)
    }
}
impl FunctionStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_function_stmt(self)
    }
}

impl WhileStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_while_stmt(self)
    }
}

impl IfStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_if_stmt(self)
    }
}

impl ExpressionStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_expression_stmt(self)
    }
}

impl BlockStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_block_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_var_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_print_stmt(self)
    }
}