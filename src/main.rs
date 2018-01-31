extern crate ansi_term;

mod parser;
mod interpreter;
mod intrinsics;
mod errors;
mod utils;
mod environment;
mod color;
mod repl;

use parser::*;
use interpreter::*;
use intrinsics::{ Intrinsics, functions };
use environment::Environment;

const MAIN_FILE: &'static str = "lib/main.rl";

fn init(env: &mut Environment) -> Result<(), String> {
    // Create context
    env.init_intrinsics();

    // Load library
    let main = MAIN_FILE.to_string();
    let args = &[Value::Str(main)];

    functions::_include(env, args)?;

    Ok(())
}

fn print_err<S: AsRef<str>>(msg: S) {
    let err = format!("ERROR: {}", msg.as_ref());
    println!("{}", color::err(err));
}

fn main() {
    let mut env = Environment::new();

    match init(&mut env) {
        Ok(_) => repl::run(&mut env),
        Err(why) => {
            print_err(why);
            print_err("Could not load standard library.");
        }
    }
}