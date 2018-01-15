use std::io::{self, BufReader, Error, Write};
use std::fs::File;

use ansi_term::Color::*;

use environment::*;
use parser::*;
use utils::*;
use sexpr::*;
use interpreter::*;
use values::*;

fn read_input_line() -> Result<String, Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn parse_line<S: AsRef<str>>(line: S) -> Result<Vec<SExpr>, String> {
    let mut parser = Parser::new();
    let bytes = line.as_ref().as_bytes();
    let mut reader = BufReader::new(bytes);
    
    parser.parse_all(&mut reader)
}

pub fn print_prompt<S: AsRef<str>>(prompt: S) -> io::Result<()> {
    print!("{}", Blue.bold().paint(prompt.as_ref()));
    io::stdout().flush()
}

pub fn print_err<S: AsRef<str>>(why: S) {
    let err = Red.paint(format!("ERROR: {}", why.as_ref()));
    println!("{}", err);
}

/// Evaluates the specified expressions. 
fn eval_exprs(env: &mut Environment, exprs: &[SExpr]) {
    for expr in exprs {
        match expr.eval(env) {
            Ok(res) => match res {
                Value::List(ref vals) if vals.is_empty() => continue,
                _ => println!("{}", res)
            },
            Err(why) => {
                print_err(why)
            }
        }
    }
}

/// Runs a REPL for the specified environment.
pub fn run(env: &mut Environment) {
    let prompt = format!("{}> ", user_name());
    loop {
        print_prompt(&prompt).expect("Failed to print prompt.");
        if let Ok(line) = read_input_line() {
            match parse_line(line) {
                Ok(ref exprs) => eval_exprs(env, exprs),
                Err(why) => print_err(why)
            }
        } else {
            print_err("Could not read input.");
        }
    }
}