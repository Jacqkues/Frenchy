use core::panic;

use crate::{
    error::ParserError,
    expr::{BinaryExpr, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr, VariableExpr, LogicalExpr, CallExp},
    stmt::{BlockStmt, ExpressionStmt, PrintStmt, Stmt, VarStmt, IfStmt, WhileStmt, FunctionStmt, ReturnStmt},
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn match_tokens(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_tokens_na(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.check(token_type) {
                
                return true;
            }
        }
        false
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(&token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParserError {
                token: self.peek().clone(),
                message: message.to_string(),
            })
        }
    }
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        return statements;
    }

    fn statement(&mut self) -> Stmt {
        match self.peek().token_type {
            TokenType::PRINT => {
                self.advance();
                return self.print_statement();
            }
            TokenType::LEFT_BRACE => {
                self.advance();
                return Stmt::Block(BlockStmt {
                    statements: self.brace_block(),
                });
            }
            TokenType::THEN => {
                self.advance();
                return Stmt::Block(BlockStmt {
                    statements: self.if_block(),
                });
            }
            TokenType::IF => {
                self.advance();
                return self.if_statement();
            }
            TokenType::WHILE => {
                self.advance();
                return self.while_statement();
            }
            TokenType::FOR => {
                self.advance();
                return self.for_statement();
            }

            TokenType::RETURN => {
                self.advance();
                return self.return_statement();
            }
            _ => {}
        }

        self.expression_statement()
    }


    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous().clone();
        let value = if !self.check(&TokenType::SEMICOLON) {
            self.expression()
        } else {
            Expr::LiteralExpr(LiteralExpr {
                value: Literal::Nil,
            })
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.")
            .unwrap();
        Stmt::Return(ReturnStmt { keyword, value:Some(value) })
    }

    fn for_statement(&mut self) -> Stmt{

       // println!("parse for");
        self.consume(TokenType::LEFT_PAREN,"Expect '(' after for.").unwrap();

        let init;

        match self.peek().token_type{
            TokenType::SEMICOLON => {
                init = None;
            }
            TokenType::VAR => {
                self.advance();
                init = Some(self.var_declaration());
            }

            _ => {
                init = Some(self.expression_statement())
            }   
        };
        let mut condition = None;

        if !self.check(&TokenType::SEMICOLON){
            condition = Some(self.expression())
        };

        self.consume(TokenType::SEMICOLON,"Expect ';' after loop condition");

        let mut increment = None;

        if !self.check(&TokenType::RIGHT_PAREN){
            increment = Some(self.expression())
        };

        self.consume(TokenType::RIGHT_PAREN,"Expect ')' after for clauses");


        


        let mut body = self.statement();

      

        if let Some(increment) = increment {
            body = Stmt::Block(BlockStmt{
                statements: vec![body, Stmt::Expression(ExpressionStmt{
                    expression: increment
                })]
            });
        };

        if let Some(condition) = condition {
            
            body = Stmt::While(WhileStmt{
                condition:condition,
                body:Box::new(body),
            });
        }

        
        if let Some(init) = init {
            
            body = Stmt::Block(BlockStmt{
                statements: vec![init,body]
            })
        }
        //println!("body : {:?}",body);
      //  panic!("test");
        body
        
    }

    fn while_statement(&mut self) -> Stmt {
       // self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.").unwrap();
        let condition = self.expression();
      //  self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.").unwrap();
        let body = self.statement();
        Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        })
    }


    fn if_statement(&mut self) -> Stmt {
        
        let condition = self.expression();
       


        
        let then_branch = self.statement();
        let else_branch = if self.match_token(TokenType::ELSE) {
            Some(Box::new(self.statement()))
        } else {
            None
        };
        Stmt::If(IfStmt {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn if_block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.match_tokens_na(vec![TokenType::ENDIF , TokenType::ELSE]) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        //self.consume(TokenType::ENDIF, "Expect 'FINSI' after if block.").unwrap();

         match self.peek().token_type {
            TokenType::ENDIF => {
                self.consume(TokenType::ENDIF, "Expect 'FINSI' after if block.").unwrap();
            }
            TokenType::ELSE => {
                //self.consume(TokenType::ENDIF, "Expect 'FINSI' after if block.").unwrap();
            }
            _ => {
                println!("{:?}" , self.peek());
                panic!("expect endif or else");
            }
        }
        statements
    }

    fn start_block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.match_tokens_na(vec![TokenType::END]) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::END, "Expect 'FINSI' after if block.").unwrap();

        
        statements  
    }

    fn brace_block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.")
            .unwrap();
        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_token(TokenType::VAR) {
            return self.var_declaration();
        }


        match self.peek().token_type{
            TokenType::VAR => {
                self.advance();
                return self.var_declaration();
            }
            TokenType::FUN => {
                self.advance();
                return self.function("function");
            }
            _ => {
                self.statement()
            }
        }

       
    }

    fn function(&mut self, kind: &str) -> Stmt {
        let name = match self.consume(TokenType::IDENTIFIER, &format!("Expect {} name.", kind)) {
            Ok(token) => token.clone(),
            Err(e) => {
                println!("{}", e);
                return Stmt::Expression(ExpressionStmt {
                    expression: Expr::LiteralExpr(LiteralExpr {
                        value: Literal::Nil,
                    }),
                });
            }
        };
        self.consume(TokenType::LEFT_PAREN, &format!("Expect '(' after {} name.", kind))
            .unwrap();
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if parameters.len() >= 255 {
                    println!("Cannot have more than 255 parameters.");
                }
                parameters.push(
                    self.consume(TokenType::IDENTIFIER, "Expect parameter name.")
                        .unwrap()
                        .clone(),
                );
                if !self.match_token(TokenType::COMMA) {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters.")
            .unwrap();
        self.consume(
            TokenType::START,
            &format!("Expect '{{' before {} body.", kind),
        )
        .unwrap();
        let body = self.start_block();
        //println!("[*] parsing function {:?}",body);
        Stmt::Function(FunctionStmt {
            name,
            params:parameters,
            body,
        })
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = match self.consume(TokenType::IDENTIFIER, "Expect variable name.") {
            Ok(token) => token.clone(),
            Err(e) => {
                println!("{}", e);
                return Stmt::Expression(ExpressionStmt {
                    expression: Expr::LiteralExpr(LiteralExpr {
                        value: Literal::Nil,
                    }),
                });
            }
        };
        let initializer = if self.match_token(TokenType::ASSIGN) {
            Some(self.expression())
        } else {
            None
        };
        match self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable déclaration.",
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                // return Stmt::Expression(ExpressionStmt { expression: Expr::LiteralExpr(LiteralExpr { value: Literal::Nil }) });
            }
        };
        Stmt::Var(VarStmt {
            name: name,
            initializer,
        })
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        match self.consume(TokenType::SEMICOLON, "Expect ';' after value.") {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                return Stmt::Expression(ExpressionStmt { expression: value });
            }
        }
        Stmt::Print(PrintStmt { expression: value })
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
     /*    match self.consume(TokenType::SEMICOLON, &format!("[stmt] Expect ';' after expression. {:?} ",expr)) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                return Stmt::Expression(ExpressionStmt { expression: expr });
            }
        }*/
        self.consume(TokenType::SEMICOLON, &format!("[stmt] Expect ';' after expression. {:?} ",expr)).unwrap();

        Stmt::Expression(ExpressionStmt { expression: expr })
    }

    fn expression(&mut self) -> Expr {
        //self.equality()
        self.assignment()
    }



    fn assignment(&mut self) -> Expr {
        let expr = self.or();
        if self.match_token(TokenType::ASSIGN) {
            let equals = self.previous().clone();
            let value = self.assignment();
            match expr {
                Expr::VariableExpr(var_expr) => {
                    return Expr::AssignVarExpr(crate::expr::AssignVarExpr {
                        name: var_expr.name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    println!("Invalid assignment target.");
                }
            }
        }
        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.match_token(TokenType::OR) {
            let operator = self.previous().clone();
            let right = self.and();
            expr = Expr::LogicalExpr(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.match_token(TokenType::AND) {
            let operator = self.previous().clone();
            let right = self.equality();
            expr = Expr::LogicalExpr(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::SLASH, TokenType::STAR, TokenType::MODULO]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::UnaryExpr(UnaryExpr {
                operator,
                right: Box::new(right),
            });
        }
        return self.call();
    }

    fn call(&mut self) -> Expr{
        let mut expr = self.primary();

        loop{
            if self.match_token(TokenType::LEFT_PAREN){
                expr = self.finish_call(&expr);
            }else{
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, expr:&Expr) -> Expr {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RIGHT_PAREN) {
            loop{

                

                arguments.push(self.expression());
                //todo verify arguments size
                if !self.match_token(TokenType::COMMA) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments.").unwrap();

        Expr::CallExpr(CallExp{
            callee: Box::new(expr.clone()),
            paren:paren.clone(),
            arguments,
        })
    }

    fn primary(&mut self) -> Expr {
        match self.peek() {
            Token {
                token_type: TokenType::FALSE,
                ..
            } => {
                self.advance();
                return Expr::LiteralExpr(LiteralExpr {
                    value: Literal::Boolean(self.previous().clone()),
                });
            }
            Token {
                token_type: TokenType::TRUE,
                ..
            } => {
                self.advance();
                return Expr::LiteralExpr(LiteralExpr {
                    value: Literal::Boolean(self.previous().clone()),
                });
            }
            Token {
                token_type: TokenType::NIL,
                ..
            } => {
                self.advance();
                return Expr::LiteralExpr(LiteralExpr {
                    value: Literal::Nil,
                });
            }
            Token {
                token_type: TokenType::NUMBER,
                ..
            } => {
                self.advance();
                return Expr::LiteralExpr(LiteralExpr {
                    value: Literal::Number(self.previous().clone()),
                });
            }
            Token {
                token_type: TokenType::STRING,
                ..
            } => {
                self.advance();
                return Expr::LiteralExpr(LiteralExpr {
                    value: Literal::String(self.previous().clone()),
                });
            }
            Token {
                token_type: TokenType::LEFT_PAREN,
                ..
            } => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")
                    .unwrap();
                return Expr::GroupingExpr(GroupingExpr {
                    expression: Box::new(expr),
                });
            }
            Token {
                token_type: TokenType::IDENTIFIER,
                ..
            } => {
                self.advance();
                return Expr::VariableExpr(VariableExpr {
                    name: self.previous().clone(),
                });
            }
            _ => {
                return Expr::LiteralExpr(LiteralExpr {
                    value: Literal::Nil,
                });
            }
        }
    }
}
