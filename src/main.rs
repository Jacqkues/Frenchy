use std::io::{self, Write};

mod error;
mod expr;
mod interpret_visitor;
mod parser;
mod print_visitor;
mod scanner;
mod token;
mod value;
mod visitor;
mod stmt;
mod environment;
mod callable;
mod builtin;
mod resolver_visitor;










fn main() {
    let mut interpreter = interpret_visitor::InterpretVisitor::new();

    loop {
        let mut input = String::new();
        let mut first_line = true;
        let mut in_a_scope = false;
        let mut in_a_function = false;
        let mut indent = 0;
        loop {
            if first_line {
                print!("> ");
                first_line = false;
            } else if in_a_function{
                print!(".... ");
            }else{
                print!(".. ");
            }
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            input.push_str(&line);

            if line.trim_end().ends_with("debut") {
               
               in_a_function = true;
            }

            if line.trim_end().ends_with("fin") {
                in_a_function = false;
                break;
            }

            if line.trim_end().ends_with(';') && !in_a_function{
                first_line = true;
                break;
            }

        }

        let mut lexer = scanner::Lexer::new(&input);
        lexer.scan_tokens();
        let mut parser = parser::Parser::new(lexer.tokens);
        let expr = parser.parse();

        {
            let mut resolver = resolver_visitor::ResolverVisitor::new(&mut interpreter);
            resolver.resolve(&expr);
        }

        let result = interpreter.interpret(&expr);
        match result {
            Ok(value) => println!("{:?}", value),
            Err(e) => println!("{}", e),
        }
    }
}
