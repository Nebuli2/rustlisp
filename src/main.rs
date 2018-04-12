extern crate ansi_term;

mod parser;
mod interpreter;
mod intrinsics;
mod errors;
mod utils;
mod color;
mod repl;

use parser::*;
use interpreter::*;
use intrinsics::*;

const ENTRY_POINT: &str = "lib/main.rl";

fn init(env: &mut Environment) -> Result<(), String> {
    // Create context
    env.init_intrinsics();

    // Load library
    let main = ENTRY_POINT.to_string();
    let args = &[Value::Str(main)];

    functions::_import(env, args)?;

    Ok(())
}

fn print_err<S>(msg: S)
where
    S: AsRef<str>,
{
    let err = format!("ERROR: {}", msg.as_ref());
    println!("{}", color::err(err));
}

fn main() {
    let mut env = Environment::default();

    init(&mut env)
        .map(|_| repl::run(&mut env))
        .unwrap_or_else(|err| {
            print_err(err);
            print_err("Could not load standard library.");
        });
}
