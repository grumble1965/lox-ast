mod error;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

use error::LoxError;
use scanner::Scanner;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};

pub fn main() {
    let args: Vec<String> = args().collect();
    // println!("args: {:?}", args);

    match args.len() {
        1 => run_prompt().expect("Could not flush stdout"),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf).is_err() {
        // Ignore: error was already reported
        std::process::exit(65);
    }

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    print!("> ");
    stdout().flush()?;
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line);
        } else {
            break;
        }
        print!("> ");
        stdout().flush()?;
    }
    Ok(())
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token)
    }

    Ok(())
}
