use super::*;
use Value::*;
use parser::Parser;
use std::io::{stdout, BufReader, Write};

use std::process::exit;

/// Represents the output of a function.
type EvalResult = Result<Value>;

/// Represents a mutable reference to an environment.
type Env<'a> = &'a mut Environment;

/// Represents a slice containing the arguments passed to a function.
type Args<'a> = &'a [Value];

/// `exit : num -> nil`
///
/// Exits the process with the specified exit code.
pub fn _exit(_: Env, args: Args) -> EvalResult {
    let len = args.len();
    let ecode = match len {
        0 => 0,
        1 => {
            let code = &args[0];
            match code {
                &Num(n) => n as i32,
                _ => return Err(not_a_number(code)),
            }
        }
        n => return Err(arity_at_most(1, n)),
    };
    exit(ecode);
}

/// `print : A... -> nil`
///
/// Prints the specified values to the standard output.
pub fn _print(env: Env, args: Args) -> EvalResult {
    let out = _concat(env, args)?;
    match out {
        Str(ref s) => {
            print!("{}", s);
            stdout().flush().expect("Failed to flush stdout.");
            ok(nil())
        }
        _ => err("Concat failed to produce a string."),
    }
}

/// `println : A... -> nil`
///
/// Prints the specified values, followed by a newline to the standard output.
pub fn _println(env: Env, args: Args) -> EvalResult {
    let out = _concat(env, args)?;
    match out {
        Str(ref s) => {
            println!("{}", s);
            ok(nil())
        }
        _ => err("Concat failed to produce a string."),
    }
}

/// `begin : A... -> A`
///
/// Produces the final values of the specified values. In practice, this
/// function evaluates all statements provided and produces the final
/// value.
pub fn _begin(_: Env, args: Args) -> EvalResult {
    if args.is_empty() {
        Ok(nil())
    } else {
        Ok(args[args.len() - 1].clone())
    }
}

/// `+ : num... -> num`
///
/// Produces the sum of 0 and the specified nums.
pub fn _add(_: Env, args: Args) -> EvalResult {
    let mut sum = 0.0;
    for arg in args.iter() {
        match arg {
            &Num(num) => sum += num,
            _ => return Err(not_a_number(arg)),
        }
    }
    ok(sum)
}

/// `- : num num... -> num`
///
/// Produces the difference between the first num and the sum of the
/// subsequent nums. If only one num is provided, the num is negated.
pub fn _sub(_: Env, args: Args) -> EvalResult {
    let len = args.len();
    if len > 0 {
        let first = &args[0];
        match first {
            &Num(n) => {
                if len == 1 {
                    Ok(Num(-n))
                } else {
                    let mut acc = n;
                    for arg in &args[1..] {
                        match arg {
                            &Num(num) => acc -= num,
                            _ => return Err(not_a_number(arg)),
                        }
                    }
                    ok(acc)
                }
            }
            _ => Err(not_a_number(first)),
        }
    } else {
        Err(arity_at_least(2, len))
    }
}

/// `* : num... num`
///
/// Produces the product of 1 and the specified values.
pub fn _mul(_: Env, args: Args) -> EvalResult {
    let mut prod = 1.0;
    for arg in args.iter() {
        match arg {
            &Num(num) => prod *= num,
            _ => return Err(not_a_number(arg)),
        }
    }
    ok(prod)
}

/// `/ : num num... -> num`
///
/// Produces the quotient between the first num and the product of the
/// subsequent nums. If only one num is provided, the num is inverted.
pub fn _div(_: Env, args: Args) -> EvalResult {
    let len = args.len();
    if len > 0 {
        let first = &args[0];
        match *first {
            Num(n) => {
                if len == 1 {
                    Ok(Num(1.0 / n))
                } else {
                    let mut acc = n;
                    for arg in &args[1..] {
                        match *arg {
                            Num(num) => acc /= num,
                            _ => return Err(not_a_number(arg)),
                        }
                    }
                    ok(acc)
                }
            }
            _ => Err(not_a_number(first)),
        }
    } else {
        Err(arity_at_least(2, len))
    }
}

/// Produces the modulo of the two specified `f64`s.
fn modulo(x: f64, y: f64) -> f64 {
    x % y
}

/// `modulo : num num -> num`
///
/// Produces the modulo of the two specified nums.
pub fn _modulo(_: Env, args: Args) -> EvalResult {
    binary_fn(args, modulo)
}

/// `sqrt : num -> num`
///
/// Produces the square root of the specified num.
pub fn _sqrt(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::sqrt)
}

/// `log : num num -> num`
///
/// Produces the logarithm of the first specified num, using the second
/// specified num as the base.
pub fn _log(_: Env, args: Args) -> EvalResult {
    binary_fn(args, f64::log)
}

/// `ln : num -> num`
///
/// Produces the natural logarithm of the specified num.
pub fn _ln(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::ln)
}

/// `pow : num num -> num`
///
/// Produces the num equal to the first num raised to the power of the
/// second num.
pub fn _pow(_: Env, args: Args) -> EvalResult {
    binary_fn(args, f64::powf)
}

/// Converts a slice of values and a function taking one `f64` into a
/// `Result<Value, String`. It checks that the number of arguments is equal to
/// one.
fn unary_fn<F>(args: Args, f: F) -> EvalResult
where
    F: Fn(f64) -> f64,
{
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Num(x) => ok(f(x)),
        _ => err(not_a_number(arg)),
    }
}

/// Converts a slice of values and a function taking two `f64`s into a
/// `Result<Value, String`. It checks that the number of arguments is equal to
/// two.
fn binary_fn<F>(args: Args, f: F) -> EvalResult
where
    F: Fn(f64, f64) -> f64,
{
    check_arity(2, args.len())?;

    let (u, v) = (&args[0], &args[1]);
    match (u, v) {
        (&Num(u), &Num(v)) => ok(f(u, v)),
        _ => err(format!("Expected (num num), found ({} {}).", u, v)),
    }
}

/// `sin : num -> num`
///
/// Produces the sine of the specified num.
pub fn _sin(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::sin)
}

/// `cos : num -> num`
///
/// Produces the cosine of the specified num.
pub fn _cos(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::cos)
}

/// `tan : num -> num`
///
/// Produces the tangent of the specified num.
pub fn _tan(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::tan)
}

/// `csc : num -> num`
///
/// Produces the cosecant of the specified num.
pub fn _csc(_: Env, args: Args) -> EvalResult {
    unary_fn(args, |x| 1.0 / f64::cos(x))
}

/// `sec : num -> num`
///
/// Produces the secant of the specified num.
pub fn _sec(_: Env, args: Args) -> EvalResult {
    unary_fn(args, |x| 1.0 / f64::cos(x))
}

/// `cot : num -> num`
///
/// Produces the cotangent of the specified num.
pub fn _cot(_: Env, args: Args) -> EvalResult {
    unary_fn(args, |x| 1.0 / f64::tan(x))
}

/// `asin : num -> num`
///
/// Produces the inverse sine of the specified num.
pub fn _asin(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::asin)
}

/// `acos : num -> num`
///
/// Produces the inverse cosine of the specified num.
pub fn _acos(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::acos)
}

/// `atan : num -> num`
///
/// Produces the inverse tangent of the specified num.
pub fn _atan(_: Env, args: Args) -> EvalResult {
    unary_fn(args, f64::atan)
}

/// `atan2 : num num -> num`
///
pub fn _atan2(_: Env, args: Args) -> EvalResult {
    binary_fn(args, f64::atan2)
}

pub fn load_trig_fns(env: Env) {
    use super::Intrinsics;

    env.define_intrinsic("sin", _sin);
    env.define_intrinsic("cos", _cos);
    env.define_intrinsic("tan", _tan);
    env.define_intrinsic("csc", _csc);
    env.define_intrinsic("sec", _sec);
    env.define_intrinsic("cot", _cot);
    env.define_intrinsic("asin", _asin);
    env.define_intrinsic("acos", _acos);
    env.define_intrinsic("atan", _atan);
    env.define_intrinsic("atan2", _atan2);
}

/// `num? : A -> bool`
///
/// Determines whether or not the specified value is a num.
pub fn _is_num(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        Num(_) => ok(true),
        _ => ok(false),
    }
}

/// `bool? : A -> bool`
///
/// Determines whether or not the specified value is a bool.
pub fn _is_bool(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        Bool(_) => ok(true),
        _ => ok(false),
    }
}

/// `str? : A -> bool`
///
/// Determines whether or not the specified value is a str.
pub fn _is_str(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        Str(_) => ok(true),
        _ => ok(false),
    }
}

/// `symbol? : A -> bool`
///
/// Determines whether or not the specified value is a symbol.
pub fn _is_symbol(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        Symbol(..) => ok(true),
        _ => ok(false),
    }
}

/// `cons? : A -> bool`
///
/// Determines whether or not the specified value is a list.
pub fn _is_cons(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        List(_) => ok(true),
        _ => ok(false),
    }
}

/// `lambda? : A -> bool`
///
/// Determines whether or not the specified value is a function.
pub fn _is_lambda(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        Intrinsic(_) => ok(true),
        Func(..) => ok(true),
        _ => ok(false),
    }
}

pub fn load_checks(env: Env) {
    env.define_intrinsic("num?", _is_num);
    env.define_intrinsic("bool?", _is_num);
    env.define_intrinsic("str?", _is_str);
    env.define_intrinsic("symbol?", _is_symbol);
    env.define_intrinsic("cons?", _is_cons);
    env.define_intrinsic("lambda?", _is_lambda);
}

/// `list : A... -> [A]`
///
/// Wraps all specified values in a list.
pub fn _list(_: Env, args: Args) -> EvalResult {
    Ok(List(Vec::from(args)))
}

/// `cons : A [A] -> [A]`
///
/// Produces a list equal to the specified list prepended by the specified
/// value.
pub fn _cons(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (car, cdr) = (&args[0], &args[1]);
    match (car, cdr) {
        (_, &List(ref vals)) => {
            let old_len = vals.len();
            let mut new_list = Vec::<Value>::with_capacity(old_len + 1);
            new_list.push(car.clone());

            for value in vals.iter() {
                new_list.push(value.clone());
            }

            Ok(List(new_list))
        }
        _ => Err(format!("{} is not a list.", car)),
    }
}

/// `car : [A] -> A`
///
/// Produces the first element of the specified list.
pub fn _car(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let list = &args[0];
    match *list {
        List(ref vals) => {
            let len = vals.len();
            if len == 0 {
                Err(format!("Cannot call car on an empty list."))
            } else {
                ok(vals[0].clone())
            }
        }
        _ => Err(format!("{} is not a list.", list)),
    }
}

/// `cdr : [A] -> A`
///
/// Produces the rest of the specified list after the first element.
pub fn _cdr(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let list = &args[0];
    match *list {
        List(ref vals) => {
            let len = vals.len();
            if len == 0 {
                Err(format!("Cannot call cdr on an empty list."))
            } else {
                let rest = &vals[1..];
                let mut new_list = Vec::<Value>::with_capacity(len - 1);
                for value in rest.iter() {
                    new_list.push(value.clone());
                }

                Ok(List(new_list))
            }
        }
        _ => Err(format!("{} is not a list.", list)),
    }
}

/// `len : [A] -> num`
///
/// Determines the length of the specified list.
pub fn _len(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let list = &args[0];
    match *list {
        List(ref vals) => ok(vals.len() as f64),
        _ => Err(format!("{} is not a list.", list)),
    }
}

/// `nth : [A] num -> A`
///
/// Produces the nth value of the specified list.
pub fn _nth(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (list, index) = (&args[0], &args[1]);
    match (list, index) {
        (&List(ref vals), &Num(num)) => {
            if vals.is_empty() {
                return ok(nil());
            }

            let index = num as usize;
            let check = index as f64;
            if check != num {
                return err("List index must be an integer.");
            }

            ok(vals[index].clone())
        }
        _ => err("Does not match contract."),
    }
}

/// `append : A [A] -> [A]`
pub fn _append(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    match (&args[0], &args[1]) {
        (value, &List(ref list)) => {
            let mut buf = list.clone();
            buf.push(value.clone());
            Ok(List(buf))
        }
        (_, list) => Err(format!("{} is not a list.", list)),
    }
}

/// `< : num num -> bool`
///
/// Determines whether or not the first argument is less than the second
/// argument.
pub fn _is_l(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    cmp(a, b)
        .map(|dif| Value::Bool(dif < 0.0))
        .ok_or(format!("Cannot compare {} to {}.", a, b))
}

/// `<= : num num -> bool`
///
/// Determines whether or not the first argument is less than or equal to
/// the second argument.
pub fn _is_le(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    cmp(a, b)
        .map(|dif| Value::Bool(dif <= 0.0))
        .ok_or(format!("Cannot compare {} to {}.", a, b))
}

/// `> : num num -> bool`
///
/// Determines whether or not the first argument is greater than the second
/// argument.
pub fn _is_g(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    cmp(a, b)
        .map(|dif| Value::Bool(dif > 0.0))
        .ok_or(format!("Cannot compare {} to {}.", a, b))
}

/// `>= : num num -> bool`
///
/// Determines whether or not the first argument is greater than or equal
/// to the second argument.
pub fn _is_ge(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    cmp(a, b)
        .map(|dif| Value::Bool(dif >= 0.0))
        .ok_or(format!("Cannot compare {} to {}.", a, b))
}

/// `eq? : A A -> bool`
///
/// Determines whether or not the two specified values are equal to one
/// another.
pub fn _is_eq(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    ok(a == b)
}

/// `or : bool... -> bool`
///
/// Produces the logical `or` of all the specified boolean values.
pub fn _or(_: Env, args: Args) -> EvalResult {
    for arg in args {
        match *arg {
            Bool(b) => if b {
                return ok(true);
            },
            _ => return err(format!("{} is not a bool.", arg)),
        }
    }

    ok(false)
}

/// `and : bool... -> bool`
///
/// Produces the logical `and` of all the specified boolean values.
pub fn _and(_: Env, args: Args) -> EvalResult {
    for arg in args {
        match *arg {
            Bool(b) => if !b {
                return ok(false);
            },
            _ => return err(format!("{} is not a bool.", arg)),
        }
    }

    ok(true)
}

/// `apply : (A... -> B) [A] -> B`
///
/// Expands the specified list of values into a variadic input for the
/// specified function, producing that function's output.
pub fn _apply(env: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (func, args) = (&args[0], &args[1]);
    match (func, args) {
        (&Func(..), &List(ref list)) => eval_func(func, list, env),
        (&Intrinsic(func), &List(ref list)) => func(env, list),
        _ => Err(format!("Contract not satisfied: {} {}.", func, args)),
    }
}

/// `not : bool -> bool`
///
/// Inverts the specified boolean value.
pub fn _not(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match *arg {
        Bool(b) => ok(!b),
        _ => Err(format!("{} is not a bool.", arg)),
    }
}

/// `A... -> str`
///
/// Produces a string containing all arguments concatenated together.
pub fn _concat(_: Env, args: Args) -> EvalResult {
    let mut buf = String::new();

    for arg in args {
        let arg_str = format!("{}", arg);
        buf.push_str(&arg_str);
    }

    ok(buf)
}

pub fn _eval(env: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = (&args[0]).clone();
    let expr: SExpr = arg.into();

    expr.eval(env)
}

/// Produces an error if the number of arguments found doesn't match the
/// number of arguments expected.
fn check_arity(expected: usize, found: usize) -> Result<()> {
    if found != expected {
        Err(arity_exact(expected, found))
    } else {
        Ok(())
    }
}

/// Compares the two specified values. If they are numbers, their difference
/// is returned. Otherwise, `None` is returned.
fn cmp(a: &Value, b: &Value) -> Option<f64> {
    match (a, b) {
        (&Num(a), &Num(b)) => Some(a - b),
        _ => None,
    }
}

#[derive(Debug)]
enum StrSection<'a> {
    Str(&'a str),
    Expr(&'a str),
}

fn split_str(s: &str) -> Result<Vec<StrSection>> {
    use self::StrSection::*;
    let mut strs = Vec::new();

    let mut in_expr = false;
    let mut last = 0 as usize;
    let mut i = 0 as usize;
    let mut last_ch = '\0';
    for ch in s.chars() {
        match ch {
            '{' if last_ch == '#' => {
                strs.push(Str(&s[last..i - 1]));
                in_expr = true;
                last = i + 1; // Begin expression after opening brace
            }
            '}' if in_expr => {
                strs.push(Expr(&s[last..i])); // Push section from expression
                in_expr = false;
                last = i + 1; // Begin next section after ending brace
            }
            _ => (),
        }
        i += 1;
        last_ch = ch;
    }
    if last != i {
        strs.push(Str(&s[last..i]));
    }

    if in_expr {
        Err("Unclosed expression while interpolating string.".to_string())
    } else {
        Ok(strs)
    }
}

fn format_str(env: Env, sections: &[StrSection]) -> EvalResult {
    use self::StrSection::*;

    let mut buf = String::new();

    for section in sections {
        match *section {
            Str(s) => buf.push_str(s),
            Expr(s) => {
                let reader = BufReader::new(s.as_bytes());
                let mut parser = Parser::new(reader);

                // Get contents
                let expr = parser.parse()?;
                env.enter_scope();
                let res = expr.eval(env)?;
                env.exit_scope();
                let res = format!("{}", res);
                buf.push_str(&res);
            }
        }
    }

    ok(buf)
}

pub fn _format(env: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let format = &args[0];
    match *format {
        Str(ref s) => {
            let sections = split_str(s)?;
            let formatted = format_str(env, &sections)?;

            ok(formatted)
        }
        _ => err(format!("{} is not a str.", format)),
    }
}

pub fn _read_line(_: Env, args: Args) -> EvalResult {
    check_arity(0, args.len())?;

    let mut buf = String::new();
    ::std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");
    let buf = buf.trim().to_string();

    ok(buf)
}

pub fn _parse(env: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let input = &args[0];
    match *input {
        Str(ref s) => {
            let s = format!("'{}", s);
            let bytes = s.as_bytes();
            let mut parser = Parser::new(BufReader::new(bytes));

            let expr = parser.parse()?;
            let val = expr.eval(env)?;

            ok(val)
        }

        _ => err(format!("{} is not a str.", input)),
    }
}

fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        n => fib(n - 1) + fib(n - 2),
    }
}

pub fn _fib_rust(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let n = &args[0];
    match *n {
        Num(n) => {
            let n = n as u64;
            let res = fib(n);
            let res = res as f64;
            ok(res)
        }
        _ => err(not_a_number(n)),
    }
}

use std::fs::File;

/// `run-file : str... -> A`
/// Opens and runs the specified file.
pub fn _include(env: Env, args: Args) -> EvalResult {
    let mut vals = Vec::<Value>::new();

    for arg in args {
        if let Str(ref file_name) = *arg {
            if let Ok(file) = File::open(file_name) {
                let reader = BufReader::new(file);
                let mut parser = Parser::new(reader);
                let exprs = parser.parse_all()?;

                let mut list = vec![SExpr::Ident("begin".to_string(), false)];
                list.extend(exprs);
                let expr = SExpr::List(list);
                let res = expr.eval(env)?;
                vals.push(res);
            } else {
                return err(format!("Could not open file {}.", file_name));
            }
        } else {
            return err(format!("{} is not a str.", arg));
        }
    }

    Ok(nil())
}

use std::io::prelude::*;

pub fn _read_file(_: Env, args: Args) -> EvalResult {
    check_arity(1, args.len())?;

    let arg = &args[0];
    if let Str(ref path) = *arg {
        // Open file
        let mut file = File::open(path).map_err(|_| format!("Could not open file {}.", path))?;

        // Read file to buffer
        let mut contents = String::from("(begin ");
        file.read_to_string(&mut contents)
            .map_err(|_| format!("Could not read file {}.", path))?;
        contents.push(')');

        Ok(Str(contents))
    } else {
        Err(format!("{} is not a str.", arg))
    }
}

/// `write-file : str str -> bool`
///
/// Attempts to write the specified string to the file with the specified path.
pub fn _write_file(_: Env, args: Args) -> EvalResult {
    check_arity(2, args.len())?;

    let (file, data) = (&args[0], &args[1]);
    match (file, data) {
        (&Str(ref path), data) => {
            let mut file =
                File::create(path).map_err(|_| format!("Could not create file {}.", path))?;
            let data_str = data.to_string();
            let data_bytes = data_str.as_bytes();
            file.write(data_bytes)
                .map_err(|_| format!("Could not write to file {}.", path))?;
            Ok(nil())
        }
        _ => Err(format!(
            "Contract not satisfied. Expected str str, found {} {}.",
            file, data
        )),
    }
}
