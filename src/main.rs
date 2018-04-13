extern crate ansi_term;

extern crate clap;
use clap::{App, Arg};

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

    let matches = App::new("RLisp")
        .version("1.0")
        .author("Benjamin Hetherington <b.w.hetherington@gmail.com>")
        .about("The RLisp language")
        .arg(Arg::with_name("interactive")
            .short("i")
            .long("config")
            .help("Toggles interactive mode"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(false)
            .index(1))
        .get_matches();

    let file_specified = matches.is_present("INPUT");
    let interactive = !file_specified || matches.is_present("interactive");
    let input = matches.value_of("INPUT").unwrap_or("lib/repl.rl");

    init(&mut env)
        .and_then(|_| {
            let args = [Value::Str(input.to_string())];
            functions::_import(&mut env, &args)?;
            if interactive {
                repl::run(&mut env);
            }
            Ok(())
        })
        .unwrap_or_else(|err| {
            print_err(err);
            print_err("Could not load standard library.");
        });
}
