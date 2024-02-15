use std::{io::Write, fs};

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
use std::process::Command;









fn main() {
  
    let _output = Command::new("clear")
        .output()
        .expect("Failed to execute command");
  /*   let input = String::from("var x = 5; afficher x; x = x + 2; afficher x;");
    let mut lexer = scanner::Lexer::new(&input);
    lexer.scan_tokens();

    for token in &lexer.tokens {
        println!("{:?}", token);
    }

    let mut parser = parser::Parser::new(lexer.tokens);

    let expr = parser.parse();

    //let mut visitor = print_visitor::PrintVisitor;
   // visitor.print(&expr);
    let mut visitor_inter = interpret_visitor::InterpretVisitor::new();
    let result = visitor_inter.interpret(&expr);
    match result {
        Ok(value) => println!("Result: {:?}", value),
        Err(e) => println!("{}", e),
    }*/


   /* let mut interpreter = interpret_visitor::InterpretVisitor::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = scanner::Lexer::new(&input);
        lexer.scan_tokens();
        let mut parser = parser::Parser::new(lexer.tokens);
        let expr = parser.parse();
        let result = interpreter.interpret(&expr);
        match result {
            Ok(value) => println!("Result: {:?}", value),
            Err(e) => println!("{}", e),
        }
    }*/


    let mut interpreter = interpret_visitor::InterpretVisitor::new();
    let mut resolver = resolver_visitor::ResolverVisitor::new(&mut interpreter);

    let filename = "/home/cytech/Desktop/Projet_Perso/Frenchy/frenchy/target/debug/main.fr";
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let mut lexer = scanner::Lexer::new(&input);
    lexer.scan_tokens();
    let mut parser = parser::Parser::new(lexer.tokens);
    let expr = parser.parse();
   // println!("{:?}",expr);
    resolver.resolve(&expr);
    let result = interpreter.interpret(&expr);
    match result {
        Ok(value) => println!("Result: {:?}", value),
        Err(e) => println!("{}", e),
    }

}
