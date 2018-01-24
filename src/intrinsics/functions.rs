use super::*;
use Value::*;
use parser::Parser;
use std::io::BufReader;
use std::io::Read;

/// Represents the output of a function.
type Output = Result<Value, String>;

/// Represents a mutable reference to an environment.
type Env<'a> = &'a mut Environment;

/// Represents a slice containing the arguments passed to a function.
type Args<'a> = &'a [Value];

/// `exit : num -> nil`
/// 
/// Exits the process with the specified exit code.
pub fn _exit(_: Env, args: Args) -> Output {
    let len = args.len();
    let ecode = match len {
        0 => 0,
        1 => {
            let code = &args[0];
            match code {
                &Num(n) => n as i32,
                _ => return Err(not_a_number(code))
            }
        }
        n => return Err(arity_at_most(1, n))
    };
    ::std::process::exit(ecode);
}

/// `print : A... -> nil`
/// 
/// Prints the specified values to the standard output.
pub fn _print(env: Env, args: Args) -> Output {
    let out = _concat(env, args)?;
    if let Str(ref s) = out {
        println!("{}", s);
        ok(nil())
    } else {
        err("Unknown failure.")
    }
}

/// `begin : A... -> A`
/// 
/// Produces the final values of the specified values. In practice, this
/// function evaluates all statements provided and produces the final
/// value.
pub fn _begin(_: Env, args: Args) -> Output {
    if args.is_empty() {
        Ok(nil())
    } else {
        Ok(args[args.len() - 1].clone())
    }
}

/// `+ : num... -> num`
/// 
/// Produces the sum of 0 and the specified nums.
pub fn _add(_: Env, args: Args) -> Output {
    let mut sum = 0.0;
    for arg in args.iter() {
        match arg {
            &Num(num) => sum += num,
            _ => return Err(not_a_number(arg))
        }
    }
    ok(sum)
}

/// - : num num... -> num
/// 
/// Produces the difference between the first num and the sum of the
/// subsequent nums. If only one num is provided, the num is negated.
pub fn _sub(_: Env, args: Args) -> Output {
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
                            _ => return Err(not_a_number(arg))
                        }
                    }
                    ok(acc)
                }
            },
            _ => Err(not_a_number(first))
        }
    } else {
        Err(arity_at_least(2, len))
    }
}

/// `* : num... num`
/// 
/// Produces the product of 1 and the specified values.
pub fn _mul(_: Env, args: Args) -> Output {
    let mut prod = 1.0;
    for arg in args.iter() {
        match arg {
            &Num(num) => prod *= num,
            _ => return Err(not_a_number(arg))
        }
    }
    ok(prod)
}

/// `/ : num num... -> num`
/// 
/// Produces the quotient between the first num and the product of the
/// subsequent nums. If only one num is provided, the num is inverted.
pub fn _div(_: Env, args: Args) -> Output {
    let len = args.len();
    if len > 0 {
        let first = &args[0];
        match first {
            &Num(n) => {
                if len == 1 {
                    Ok(Num(1.0 / n))
                } else {
                    let mut acc = n;
                    for arg in &args[1..] {
                        match arg {
                            &Num(num) => acc /= num,
                            _ => return Err(not_a_number(arg))
                        }
                    }
                    ok(acc)
                }
            },
            _ => Err(not_a_number(first))
        }
    } else {
        Err(arity_at_least(2, len))
    }
}

/// `modulo : num num -> num`
/// 
/// Produces the modulo of the two specified nums.
pub fn _modulo(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    match (a, b) {
        (&Num(a), &Num(b)) => ok(a % b),
        _ => Err(format!("\"modulo\" must be passed nums."))
    }
}

/// `sqrt : num -> num`
/// 
/// Produces the square root of the specified num.
pub fn _sqrt(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let a = &args[0];
    match a {
        &Num(a) => ok(f64::sqrt(a)),
        _ => Err(format!("\"sqrt\" must be passed a num."))
    }
}

/// `pow : num num -> num`
/// 
/// Produces the num equal to the first num raised to the power of the
/// second num.
pub fn _pow(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let args = (&args[0], &args[1]);
    match args {
        (&Num(a), &Num(b)) => ok(f64::powf(a, b)),
        _ => Err(format!("\"pow\" must be passed nums."))
    }
}

/// `num? : A -> bool`
/// 
/// Determines whether or not the specified value is a num.
pub fn _is_num(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Num(_) => ok(true),
        _ => ok(false)
    }
}

/// `bool? : A -> bool`
/// 
/// Determines whether or not the specified value is a bool.
pub fn _is_bool(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Bool(_) => ok(true),
        _ => ok(false)
    }
}

/// `str? : A -> bool`
/// 
/// Determines whether or not the specified value is a str.
pub fn _is_str(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Str(_) => ok(true),
        _ => ok(false)
    }
}

/// `symbol? : A -> bool`
/// 
/// Determines whether or not the specified value is a symbol.
pub fn _is_symbol(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Symbol(_, _) => ok(true),
        _ => ok(false)
    }
}

/// `cons? : A -> bool`
/// 
/// Determines whether or not the specified value is a list.
pub fn _is_cons(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &List(_) => ok(true),
        _ => ok(false)
    }
}

/// `lambda? : A -> bool`
/// 
/// Determines whether or not the specified value is a function.
pub fn _is_lambda(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Intrinsic(_) => ok(true),
        &Func(_, _, _) => ok(true),
        _ => ok(false)
    }
}

/// `list : A... -> [A]`
/// 
/// Wraps all specified values in a list.
pub fn _list(_: Env, args: Args) -> Output {
    Ok(List(Vec::from(args)))
}

/// `cons : A [A] -> [A]`
/// 
/// Produces a list equal to the specified list prepended by the specified
/// value.
pub fn _cons(_: Env, args: Args) -> Output {
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
        },
        _ => Err(format!("{} is not a list.", car))
    }
}

/// `car : [A] -> A`
/// 
/// Produces the first element of the specified list.
pub fn _car(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let list = &args[0];
    match list {
        &List(ref vals) => {
            let len = vals.len();
            if len == 0 {
                Err(format!("Cannot call car on an empty list."))
            } else {
                ok(vals[0].clone())
            }
        },
        _ => Err(format!("{} is not a list.", list))
    }
}

/// `cdr : [A] -> A`
/// 
/// Produces the rest of the specified list after the first element.
pub fn _cdr(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let list = &args[0];
    match list {
        &List(ref vals) => {
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
        },
        _ => Err(format!("{} is not a list.", list))
    }
}

/// `len : [A] -> num`
/// 
/// Determines the length of the specified list.
pub fn _len(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let list = &args[0];
    match list {
        &List(ref vals) => ok(vals.len() as f64),
        _ => Err(format!("{} is not a list.", list))
    }
}

/// `nth : [A] num -> A`
/// 
/// Produces the nth value of the specified list.
pub fn _nth(_: Env, args: Args) -> Output {
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
        },
        _ => err("Does not match contract.")
    }
}

/// `append : [A]... -> [A]`
pub fn _append(_: Env, args: Args) -> Output {
    let mut buf = Vec::<Value>::new();

    for arg in args.iter() {
        match arg {
            &List(ref vals) => {
                let len = vals.len();
                if len > 1 {
                    let before_last = &vals[..len - 1];
                    let mut vals: Vec<_> = before_last.iter()
                        .map(|val| val.clone())
                        .collect();
                    // let last = &vals[len - 1];
                    // match last {
                    //     &List(ref last) => vals.
                    // }
                }
                // buf.extend(vals);
            },
            _ => return err(format!("{} is not a list.", arg))
        }
    }

    Ok(List(buf))
}

/// `< : num num -> bool`
/// 
/// Determines whether or not the first argument is less than the second
/// argument.
pub fn _is_l(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    let cmp = cmp(a, b);
    match cmp {
        Some(dif) => ok(dif < 0.0),
        _ => Err(format!("Cannot compare {} and {}.", a, b))
    }
}

/// `<= : num num -> bool`
/// 
/// Determines whether or not the first argument is less than or equal to
/// the second argument.
pub fn _is_le(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    let cmp = cmp(a, b);
    match cmp {
        Some(dif) => ok(dif <= 0.0),
        _ => Err(format!("Cannot compare {} and {}.", a, b))
    }
}

/// `> : num num -> bool`
/// 
/// Determines whether or not the first argument is greater than the second
/// argument.
pub fn _is_g(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    let cmp = cmp(a, b);
    match cmp {
        Some(dif) => ok(dif > 0.0),
        _ => Err(format!("Cannot compare {} and {}.", a, b))
    }
}

/// `>= : num num -> bool`
/// 
/// Determines whether or not the first argument is greater than or equal
/// to the second argument.
pub fn _is_ge(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    let cmp = cmp(a, b);
    match cmp {
        Some(dif) => ok(dif >= 0.0),
        _ => Err(format!("Cannot compare {} and {}.", a, b))
    }
}

/// `eq? : A A -> bool`
/// 
/// Determines whether or not the two specified values are equal to one
/// another.
pub fn _is_eq(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?; 

    let (a, b) = (&args[0], &args[1]);
    ok(a == b)
}

/// `or : bool bool -> bool`
/// 
/// Produces the logical `or` of the two boolean values.
pub fn _or(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    match (&args[0], &args[1]) {
        (&Bool(a), &Bool(b)) => ok(a || b),
        _ => Err(format!("\"or\" may only be called on bool values."))
    }
}

/// `and : bool bool -> bool`
/// 
/// Produces the logical `and` of the two boolean values.
pub fn _and(_: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (a, b) = (&args[0], &args[1]);
    match (a, b) {
        (&Bool(a), &Bool(b)) => ok(a && b),
        _ => Err(format!("\"and\" may only be called on bool values."))
    }
}

/// `apply : (A... -> B) [A] -> B`
/// 
/// Expands the specified list of values into a variadic input for the
/// specified function, producing that function's output.
pub fn _apply(env: Env, args: Args) -> Output {
    check_arity(2, args.len())?;

    let (func, args) = (&args[0], &args[1]);
    match (func, args) {
        (&Func(_, _, _), &List(ref list)) => eval_func(func, &list, env),
        (&Intrinsic(func), &List(ref list)) => func(env, list),
        _ => Err(format!("Contract not satisfied: {} {}.", func, args))
    }
}

/// `not : bool -> bool`
/// 
/// Inverts the specified boolean value.
pub fn _not(_: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = &args[0];
    match arg {
        &Bool(b) => ok(!b),
        _ => Err(format!("{} is not a bool.", arg))
    }
}

/// `A... -> str`
/// Produces a string containing all arguments concatenated together.
pub fn _concat(_: Env, args: Args) -> Output {
    let mut buf = String::new();

    for arg in args.iter() {
        let arg_str = format!("{}", arg);
        buf.push_str(&arg_str);
    }

    ok(buf)
}

pub fn _eval(env: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let arg = (&args[0]).clone();
    let expr: SExpr = arg.into();
    expr.eval(env)
}

/// Produces an error if the number of arguments found doesn't match the
/// number of arguments expected.
fn check_arity(expected: usize, found: usize) -> Result<(), String> {
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
        _ => None
    }
}

#[derive(Debug)]
enum StrSection<'a> {
    Str(&'a str),
    Expr(&'a str)
}

fn split_str(s: &str) -> Result<Vec<StrSection>, String> {
    use self::StrSection::*;
    let mut strs = Vec::new();

    let mut enter_expr = false;
    let mut in_expr = false;
    let mut last: usize = 0;
    let mut i: usize = 0;
    for ch in s.chars() {
        match ch {
            '$' => enter_expr = true,
            '{' if enter_expr => {
                strs.push(Str(&s[last..i - 1]));
                enter_expr = false;
                in_expr = true;
                last = i + 1; // Begin expression after opening brace
            }
            '}' if in_expr => {
                strs.push(Expr(&s[last..i])); // Push section from expression
                in_expr = false;
                last = i + 1; // Begin next section after ending brace
            },
            _ => enter_expr = false,
        }
        i += 1;
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

fn format_str(env: Env, sections: &[StrSection]) -> Output {
    use self::StrSection::*;

    let mut buf = String::new();

    for section in sections {
        match section {
            &Str(s) => buf.push_str(s),
            &Expr(s) => {
                let mut parser = Parser::new();
                let mut reader = BufReader::new(s.as_bytes());

                // Get contents
                let expr = parser.parse(&mut reader)?;
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

pub fn _format(env: Env, args: Args) -> Output {
    check_arity(1, args.len())?;

    let format = &args[0];
    match format {
        &Str(ref s) => {
            let sections = split_str(s)?;
            let formatted = format_str(env, &sections)?;

            ok(formatted)
        },
        _ => err(format!("{} is not a str.", format))
    }
}