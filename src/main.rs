use std::env;
use std::fs;
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

/// Exécute un fichier .fr donné en argument
fn run_file(path: &str) -> Result<(), String> {
    if !path.ends_with(".fr") {
        return Err(format!("Le fichier doit avoir l'extension .fr : {}", path));
    }

    let contenu = fs::read_to_string(path)
        .map_err(|e| format!("Impossible de lire '{}': {}", path, e))?;

    let mut interpreter = interpret_visitor::InterpretVisitor::new();

    let mut lexer = scanner::Lexer::new(&contenu);
    lexer.scan_tokens();

    let mut parser = parser::Parser::new(lexer.tokens);
    let expr = parser.parse();

    {
        let mut resolver = resolver_visitor::ResolverVisitor::new(&mut interpreter);
        resolver.resolve(&expr);
    }

    match interpreter.interpret(&expr) {
        Ok(value) => {
            println!("{:?}", value);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn run_repl() {
    let mut interpreter = interpret_visitor::InterpretVisitor::new();
    

    loop {
        let mut input = String::new();
        let mut first_line = true;
        let mut in_a_function = false;

        loop {
            if first_line {
                print!("> ");
                first_line = false;
            } else if in_a_function {
                print!(".... ");
            } else {
                print!(".. ");
            }
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                println!("Erreur de lecture. Fin du REPL.");
                return;
            }

            input.push_str(&line);

            let trimmed = line.trim_end();
            if trimmed.ends_with("debut") {
                in_a_function = true;
            }
            if trimmed.ends_with("fin") {
                in_a_function = false;
                break;
            }
            if trimmed.ends_with(';') && !in_a_function {
                break;
            }
        }

        // Lexing
        let mut lexer = scanner::Lexer::new(&input);
        lexer.scan_tokens();
        // Parsing
        let mut parser = parser::Parser::new(lexer.tokens);
        let expr = parser.parse();
        // Résolution
        {
            let mut resolver = resolver_visitor::ResolverVisitor::new(&mut interpreter);
            resolver.resolve(&expr);
        }
        // Interprétation
        match interpreter.interpret(&expr) {
            Ok(value) => println!("{:?}", value),
            Err(e) => println!("Erreur: {}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            run_repl();
        }
        2 => {
            let script = &args[1];
            if let Err(err_msg) = run_file(script) {
                eprintln!("Erreur : {}", err_msg);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage : {} [<script.fr>]", args[0]);
            std::process::exit(1);
        }
    }
}
