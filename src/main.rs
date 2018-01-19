extern crate ansi_term;

mod parser;
mod sexpr;
mod interpreter;
mod intrinsics;
mod errors;
mod utils;
mod values;
mod environment;
mod color;
mod repl;

use parser::Parser;
use sexpr::SExpr;
use interpreter::*;
use intrinsics::Intrinsics;
use values::*;
use environment::Environment;
use repl::*;
use std::io::BufReader;
use std::fs::File;

fn main() {

    // Create context
    let mut env = Environment::new();
    env.init_intrinsics();

    // Read library
    let _ = File::open("lisp-lib/stdlib.rlisp").map(|f| {
        let mut reader = BufReader::new(f);
        let mut parser = Parser::new();
        let parsed = parser.parse_all(&mut reader)
            .unwrap_or_else(|err| {
                let message = format!("ERROR: {}", err);
                println!("{}", (message));
                vec![]
            });
        for expr in &parsed {
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

    run(&mut env);
}