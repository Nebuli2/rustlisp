#![allow(unknown_lints)]
#![warn(clippy)]

extern crate ansi_term;

extern crate clap;
use clap::{App, Arg};

mod color;
mod err;
mod errors;
mod interpreter;
mod intrinsics;
mod parser;
mod repl;
mod utils;

use err::*;
use interpreter::*;
use intrinsics::*;
use parser::*;

use std::env;

const ENTRY_POINT: &str = "loader.rl";

fn init(lisp_env: &mut Environment, lib: &str) -> Result<(), RLError> {
    let prev = env::current_dir()?;

    // Set directory to library
    env::set_current_dir(lib)?;

    // Create context
    lisp_env.init_intrinsics();

    let args = [Value::Str(ENTRY_POINT.to_string())];
    functions::_import(lisp_env, &args)?;

    // Reset the directory
    env::set_current_dir(prev)?;

    Ok(())
}

fn print_err(msg: impl AsRef<str>) {
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

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut lisp_env = Environment::default();
    let matches = match_args();
    let file_specified = matches.is_present("INPUT");
    let interactive = !file_specified || matches.is_present("interactive");
    let input = matches.value_of("INPUT");
    let lib = matches
        .value_of("LIB_LOC")
        .unwrap_or("/Users/bwh/rust-workspace/rust-lisp/lib/");

    init(&mut lisp_env, lib)?;

    if let Some(input) = input {
        let args = [Value::Str(input.to_string())];
        functions::_import(&mut lisp_env, &args)?;
    }

    if interactive {
        repl::run(&mut lisp_env);
    }

    Ok(())
}
