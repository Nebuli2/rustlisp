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

use std::env::args;

const ENTRY_POINT: &str = "lib/loader.rl";

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
        .and_then(|_| {
            let arg_list: Vec<_> = args().skip(1).collect();
            // println!("{:?}", arg_list);
            if arg_list.is_empty() {
                let args = [Value::Str("lib/repl.rl".to_string())];
                functions::_import(&mut env, &args)?;
                repl::run(&mut env);
                Ok(())
            } else {
                let arg_list: Vec<_> = arg_list.into_iter().map(|arg| Value::Str(arg)).collect();
                functions::_import(&mut env, &arg_list).map(|_| ())
            }
        })
        .unwrap_or_else(|err| {
            print_err(err);
            print_err("Could not load standard library.");
        });
}
