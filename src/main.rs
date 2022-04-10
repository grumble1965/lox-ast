mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod token;
mod token_type;

use ast_printer::*;
use error::*;
use interpreter::*;
use parser::*;
use scanner::*;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};

pub fn main() {
    let args: Vec<String> = args().collect();
    let lox = Lox::new();
    // println!("args: {:?}", args);

    match args.len() {
        1 => lox.run_prompt().expect("Could not flush stdout"),
        2 => lox.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    _printer: AstPrinter,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            _printer: AstPrinter {},
            interpreter: Interpreter {},
        }
    }

    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&self) -> io::Result<()> {
        let stdin = io::stdin();
        print!("> ");
        stdout().flush()?;
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line);
            } else {
                break;
            }
            print!("> ");
            stdout().flush()?;
        }
        Ok(())
    }

    pub fn run(&self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            None => {}
            Some(expr) => {
                // let printer1 = AstPrinter {};
                // println!("AstPrint: {}", printer1.print(&expr)?);
                match self.interpreter.interpret(&expr) {
                    Ok(_) => println!("yay"),
                    Err(_) => println!("boo"),
                }
            }
        }
        Ok(())
    }
}
