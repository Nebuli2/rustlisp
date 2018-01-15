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

mod color;

mod repl;
use repl::*;

use std::io::BufReader;
use std::fs::File;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {

    // Create context
    let mut env = Environment::new();
    env.init_intrinsics();

    // Read library
    let _ = File::open("lisp-lib/stdlib.rkt").map(|f| {
        let mut reader = BufReader::new(f);
        let mut parser = Parser::new();
        let parsed = parser.parse_all(&mut reader)
            .unwrap_or_else(|err| {
                let message = format!("ERROR: {}", err);
                println!("{}", (message));
                vec![]
            });
        for expr in parsed.iter() {
            let res = expr.eval(&mut env);
            match res {
                Ok(val) => match val {
                    Value::List(ref l) if l.len() == 0 => continue,
                    _ => println!("{}", val)
                },
                Err(why) => {
                    let message = format!("ERROR: {}", why);
                    println!("{}", color::err(message));
                }
            }
        }
    });

    println!("Welcome to {} v{}.", NAME, VERSION);
    run(&mut env);
}