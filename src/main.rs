extern crate ansi_term;
use ansi_term::Color::*;

mod parser;
use parser::Parser;

mod sexpr;
use sexpr::SExpr;

mod interpreter;
use interpreter::*;

mod intrinsics;
use intrinsics::Intrinsics;

mod errors;
mod utils;
mod values;
use values::*;
mod environment;
use environment::Environment;

use std::io::{self, BufReader, Error, Write};
use std::fs::File;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn read_input_line() -> Result<String, Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn repl(ctx: &mut Environment) {
    loop {
        let prompt = format!("{}>", utils::user_name());
        print!("{} ", Blue.bold().paint(prompt));
        let _ = std::io::stdout().flush();
        if let Ok(line) = read_input_line() {
            let mut parser = Parser::new();
            let mut reader = BufReader::new(line.as_bytes());
            let parsed = parser.parse_all(&mut reader).unwrap_or_else(|e| {
                let err = Red.paint(format!("ERROR: {}", e));
                println!("{}", err);
                vec![SExpr::Nil]
            });

            for exp in parsed.iter() {
                match exp.eval(ctx) {
                    Ok(res) => match res {
                        Value::List(ref l) if l.is_empty() => continue,
                        _ => println!("{}", res)
                    },
                    Err(err) => {
                        let err = format!("ERROR: {}", err);
                        println!("{}", Red.paint(err));
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    // Create context
    let mut ctx = Environment::new();
    ctx.init_intrinsics();

    // Read sample file
    let _ = File::open("lisp-lib/stdlib.rkt").map(|f| {
        let mut reader = BufReader::new(f);
        let mut parser = Parser::new();
        let parsed = parser.parse_all(&mut reader)
            .unwrap_or_else(|err| {
                let message = format!("ERROR: {}", err);
                println!("{}", Red.paint(message));
                vec![]
            });
        for expr in parsed.iter() {
            let res = expr.eval(&mut ctx);
            match res {
                Ok(val) => match val {
                    Value::List(ref l) if l.len() == 0 => continue,
                    _ => println!("{}", val)
                },
                Err(why) => {
                    let message = format!("ERROR: {}", why);
                    println!("{}", Red.paint(message));
                }
            }
        }
    });

    println!("Welcome to {} v{}.", NAME, VERSION);
    repl(&mut ctx);
}