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

const ENTRY_POINT: &str = "loader.rl";

fn init(env: &mut Environment, lib: &str) -> Result<(), String> {
    // Create context
    env.init_intrinsics();

    // Load library
    let lib = if lib.ends_with("/") {
        lib.to_string()
    } else {
        format!("{}/", lib)
    };
    let main = format!("{}{}", lib, ENTRY_POINT);
    println!("Main: {}", main);
    let args = &[Value::Str(main)];

    functions::_import(env, args)?;

    Ok(())
}

fn print_err<S>(msg: S)
where
    S: AsRef<str>,
{
    let err = format!("ERROR:\n{}", msg.as_ref());
    println!("{}", color::err(err));
}

fn match_args<'a>() -> clap::ArgMatches<'a> {
    App::new("RLisp")
        .version("1.0")
        .author("Benjamin Hetherington <b.w.hetherington@gmail.com>")
        .about("The RLisp language")
        .arg(
            Arg::with_name("lib")
                .short("l")
                .long("lib")
                .value_name("LIB_LOC")
                .help("Specifies the standard library location"),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Toggles interactive mode"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .index(1),
        )
        .get_matches()
}

fn main() {
    let mut env = Environment::default();

    let matches = match_args();

    let file_specified = matches.is_present("INPUT");
    let interactive = !file_specified || matches.is_present("interactive");
    let input = matches.value_of("INPUT").unwrap_or("lib/repl.rl");
    let lib = matches
        .value_of("LIB_LOC")
        .unwrap_or("/Users/bwh/rust-workspace/rust-lisp/lib/");

    println!("{:?}", matches);

    init(&mut env, lib)
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
