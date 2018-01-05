use interpreter::*;
use parser::Parser;
use sexpr::SExpr;
use errors::*;
use values::*;
use environment::Environment;

const RESERVED_WORDS: [&'static str; 6] = [
    "define",
    "begin",
    "cond",
    "else",
    "if",
    "let"
];

fn nil() -> Value {
    Value::List(vec![])
}

pub trait Intrinsics {
    fn define_intrinsic<S: Into<String>>(
        &mut self, 
        ident: S,
        f: IntrinsicFunc
    );

    fn define_macro<S: Into<String>>(
        &mut self, 
        ident: S, 
        f: MacroFunc
    );

    fn init_intrinsics(&mut self);
}

impl Intrinsics for Environment {
    fn define_intrinsic<S: Into<String>>(
        &mut self,
        ident: S,
        f: IntrinsicFunc)  
    {
        self.define(ident, Value::Intrinsic(f));
    }

    fn define_macro<S: Into<String>>(
        &mut self, 
        ident: S,
        f: MacroFunc)
    {
        self.define(ident, Value::Macro(f));
    }

    fn init_intrinsics(&mut self) {
        // Constancs
        self.define("empty", nil());

        let infinity = ::std::f64::INFINITY;
        self.define("infinity", Value::Num(infinity));
        self.define("-infinity", Value::Num(-infinity));

        // MACROS

        // Define macro
        // (define ident value)
        // (define (func-ident param ...) body)
        self.define_macro("define", |env, exprs| {
            let len = exprs.len();
            if len == 3 {
                let (ident, val) = (&exprs[1], &exprs[2]);
                match *ident {
                    // Define variable
                    SExpr::Ident(ref s) => {
                        if RESERVED_WORDS.contains(&s.as_str()) {
                            Err(reserved_word(s))
                        } else {
                            let val = val.eval(env)?;
                            env.define(s.clone(), val);
                            ok(nil())
                        }
                    },

                    // Define function
                    SExpr::List(ref vals) => {
                        let len = vals.len();
                        if len == 0 {
                            Err(format!("Cannot redefine empty list."))
                        } else {
                            let ident = &vals[0];
                            let mut params = Vec::<String>::with_capacity(len - 1);
                            let body = val.clone();
                            for param in &vals[1..] {
                                match *param {
                                    SExpr::Ident(ref s) => params.push(s.clone()),
                                    _ => return Err(not_an_identifier(param))
                                }
                            }
                            let func = Value::Func(params, body);
                            match *ident {
                                SExpr::Ident(ref s) => {
                                    if RESERVED_WORDS.contains(&s.as_str()) {
                                        Err(reserved_word(s))
                                    } else {
                                        env.define(s.clone(), func);
                                        ok(nil())
                                    }
                                },
                                _ => Err(not_an_identifier(ident))
                            }
                        }
                    },
                    _ => Err(not_an_identifier(ident))
                }
            } else {
                Err(arity_exact(2, len - 1))
            }
        });

        // Lambda macro
        self.define_macro("lambda", |_, exprs| {
            let len = exprs.len();

            if len != 3 {
                return Err(arity_exact(2, len - 1));
            }

            let (params, body) = (&exprs[1], &exprs[2]);
            match *params {
                SExpr::List(ref params) => {
                    let mut names = Vec::<String>::with_capacity(params.len());
                    for param in params.iter() {
                        match param {
                            &SExpr::Ident(ref s) => names.push(s.to_string()),
                            _ => return Err(not_an_identifier(param))
                        }
                    }
                    Ok(Value::Func(names, body.clone()))
                },
                _ => Err(not_a_list(params))
            }
        });

        // Defun macro
        // self.define_macro("defun", |env, exprs| {
        //     let len = exprs.len();

        //     if len != 4 {
        //         return Err(arity_exact(3, len - 1));
        //     }

        //     let mut parser = Parser::new();
        //     let (ident, params, body) = (&exprs[1], &exprs[2], &exprs[3]);
        //     let format = format!(
        //         "(define {} (lambda {} {}))",
        //         ident,
        //         params,
        //         body
        //     );
        //     let parsed = parser.parse_from_str(&format)?;
        //     parsed.eval(env)
        // });

        // If macro
        self.define_macro("if", |env, exprs| {
            let len = exprs.len();

            if len != 4 {
                return Err(arity_exact(3, len - 1));
            }

            let (cond, then, other) = (&exprs[1], &exprs[2], &exprs[3]);
            let cond = match cond.eval(env)? {
                Value::Bool(cond) => cond,
                _ => return Err(not_a_bool(&cond))
            };

            if cond {
                then.eval(env)
            } else {
                other.eval(env)
            }
        });

        // (cond [condition value]
        //        ...)
        // Steps through the condition expressions. If one of the conditions
        // evaluates to true, its value is returned. Otherwise, the next
        // next expression is checked, etc.
        self.define_macro("cond", |env, exprs| {
            let conditions = &exprs[1..];
            env.enter_scope();
            env.define("else", Value::Bool(true));
            for condition in conditions.iter() {
                match *condition {
                    SExpr::List(ref vals) => {
                        let len = vals.len();
                        match len {
                            2 => {
                                let condition = vals[0].eval(env)?;
                                if let Value::Bool(b) = condition {
                                    if b {
                                        env.exit_scope();
                                        return vals[1].eval(env);
                                    }
                                } else {
                                    env.exit_scope();
                                    return Err(format!("{} is not a bool.", condition))
                                }
                            },
                            n => {
                                env.exit_scope();
                                return Err(arity_exact(2, n))
                            }
                        }
                    },
                    _ => {
                        env.exit_scope();
                        return Err(not_a_list(condition))
                    }
                }
            }
            env.exit_scope();
            ok(nil())
        });

        // Let macro
        // (let ([foo value]
        //       [bar value] 
        //       ...)
        //  (baz foo bar))
        self.define_macro("let", |env, exprs| {
            let len = exprs.len() - 1;
            if len != 2 {
                return Err(arity_exact(2, len));
            }

            let args = (&exprs[1], &exprs[2]);
            match args {
                (&SExpr::List(ref bindings), body) => {
                    env.enter_scope();
                    for expr in bindings.iter() {
                        match *expr {
                            SExpr::List(ref binding) => {
                                let len = binding.len();
                                if len != 2 {
                                    return Err(arity_exact(2, len));
                                }

                                let binding = (&binding[0], &binding[1]);
                                match binding {
                                    (&SExpr::Ident(ref s), expr) => {
                                        let res = expr.eval(env)?;
                                        env.define(s.clone(), res);
                                    },
                                    _ => {
                                        env.exit_scope();
                                        return Err(not_an_identifier(binding.0));
                                    }
                                }
                            },
                            _ => {
                                env.exit_scope();
                                return Err(not_a_list(expr));
                            }
                        }
                    }

                    let res = body.eval(env);  
                    env.exit_scope();
                    res                  
                },
                _ => Err(not_a_list(args.0))
            }
        });


        // INTRINSICS

        // Exit function
        self.define_intrinsic("exit", |_, args| {
            let len = args.len();
            let ecode = match len {
                0 => 0,
                1 => {
                    let code = &args[0];
                    match code {
                        &Value::Num(n) => n as i32,
                        _ => return Err(not_a_number(code))
                    }
                }
                n => return Err(arity_at_most(1, n))
            };
            ::std::process::exit(ecode);
        });

        // Add function
        self.define_intrinsic("+", |_, args| {
            let mut sum = 0.0;
            for arg in args.iter() {
                match arg {
                    &Value::Num(num) => sum += num,
                    _ => return Err(not_a_number(arg))
                }
            }
            ok(sum)
        });

        // Subtract function
        self.define_intrinsic("-", |_, args| {
            let len = args.len();
            if len > 0 {
                let first = &args[0];
                match first {
                    &Value::Num(n) => {
                        if len == 1 {
                            Ok(Value::Num(-n))
                        } else {
                            let mut acc = n;
                            for arg in &args[1..] {
                                match arg {
                                    &Value::Num(num) => acc -= num,
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
        });

        // Mult function
        self.define_intrinsic("*", |_, args| {
            let mut prod = 1.0;
            for arg in args.iter() {
                match arg {
                    &Value::Num(num) => prod *= num,
                    _ => return Err(not_a_number(arg))
                }
            }
            ok(prod)
        });

        // Div function
        self.define_intrinsic("/", |_, args| {
            let len = args.len();
            if len > 0 {
                let first = &args[0];
                match first {
                    &Value::Num(n) => {
                        if len == 1 {
                            Ok(Value::Num(1.0 / n))
                        } else {
                            let mut acc = n;
                            for arg in &args[1..] {
                                match arg {
                                    &Value::Num(num) => acc /= num,
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
        });

        // Begin function
        self.define_intrinsic("begin", |_, args| {
            if args.is_empty() {
                Ok(nil())
            } else {
                Ok(args[args.len() - 1].clone())
            }
        });

        // Print function
        self.define_intrinsic("print", |_, args| {
            for arg in args {
                print!("{}", arg);
            }
            println!();

            Ok(nil())
        });

        // Modulo function
        self.define_intrinsic("modulo", |_, args| {
            let len = args.len();
            if len != 2 {
                Err(arity_exact(2, len))
            } else {
                let (a, b) = (&args[0], &args[1]);
                let a = match a {
                    &Value::Num(n) => n,
                    _ => return Err(not_a_number(a))
                };
                let b = match b {
                    &Value::Num(n) => n,
                    _ => return Err(not_a_number(b))
                };
                ok(a % b)
            }
        });

        // TYPE CHECKING FUNCS

        // num?
        self.define_intrinsic("num?", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let arg = &args[0];
            match arg {
                &Value::Num(_) => ok(true),
                _ => ok(false)
            }
        });

        // bool?
        self.define_intrinsic("bool?", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let arg = &args[0];
            match arg {
                &Value::Bool(_) => ok(true),
                _ => ok(false)
            }
        });

        // str?
        self.define_intrinsic("str?", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let arg = &args[0];
            match arg {
                &Value::Str(_) => ok(true),
                _ => ok(false)
            }
        });

        // cons?
        self.define_intrinsic("cons?", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let arg = &args[0];
            match arg {
                &Value::List(_) => ok(true),
                _ => ok(false)
            }
        });

        // lambda?
        self.define_intrinsic("lambda?", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let arg = &args[0];
            match arg {
                &Value::Intrinsic(_) => ok(true),
                &Value::Func(_, _) => ok(true),
                _ => ok(false)
            }
        });

        // list
        self.define_intrinsic("list", |_, args| {
            Ok(Value::List(args.clone()))
        });

        // cons
        self.define_intrinsic("cons", |_, args| {
            let len = args.len();
            if len != 2 {
                return Err(arity_exact(2, len));
            }

            let (car, cdr) = (&args[0], &args[1]);
            match (car, cdr) {
                (_, &Value::List(ref vals)) => {
                    let old_len = vals.len();
                    let mut new_list = Vec::<Value>::with_capacity(old_len + 1);
                    new_list.push(car.clone());
                    
                    for value in vals.iter() {
                        new_list.push(value.clone());
                    }

                    Ok(Value::List(new_list))
                },
                _ => Err(format!("{} is not a list.", car))
            }
        });

        // car
        self.define_intrinsic("car", |_, args| {
            let len = args.len();
            if len != 1 {
                Err(arity_exact(1, len))
            } else {
                let list = &args[0];
                match *list {
                    Value::List(ref vals) => {
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
        });

        // cdr
        self.define_intrinsic("cdr", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let list = &args[0];
            match *list {
                Value::List(ref vals) => {
                    let len = vals.len();
                    if len == 0 {
                        Err(format!("Cannot call cdr on an empty list."))
                    } else {
                        let rest = &vals[1..];
                        let mut new_list = Vec::<Value>::with_capacity(len - 1);
                        for value in rest.iter() {
                            new_list.push(value.clone());
                        }

                        Ok(Value::List(new_list))
                    }
                },
                _ => Err(format!("{} is not a list.", list))
            }
        });

        // len
        self.define_intrinsic("len", |_, args| {
            let len = args.len();
            if len != 1 {
                return Err(arity_exact(1, len));
            }

            let list = &args[0];
            match *list {
                Value::List(ref vals) => ok(vals.len() as f64),
                _ => Err(format!("{} is not a list.", list))
            }
        });

        // <
        self.define_intrinsic("<", |_, args| {
            let len = args.len();
            if len != 2 {
                return Err(arity_exact(2, len));
            }

            let (a, b) = (&args[0], &args[1]);
            let cmp = cmp(a, b);
            match cmp {
                Some(dif) => ok(dif < 0.0),
                _ => Err(format!("Cannot compare {} and {}.", a, b))
            }
        });

        // <=
        self.define_intrinsic("<=", |_, args| {
            let len = args.len();
            if len != 2 {
                return Err(arity_exact(2, len));
            }

            let (a, b) = (&args[0], &args[1]);
            let cmp = cmp(a, b);
            match cmp {
                Some(dif) => ok(dif <= 0.0),
                _ => Err(format!("Cannot compare {} and {}.", a, b))
            }
        });

        // >
        self.define_intrinsic(">", |_, args| {
            let len = args.len();
            if len != 2 {
                return Err(arity_exact(2, len));
            }

            let (a, b) = (&args[0], &args[1]);
            let cmp = cmp(a, b);
            match cmp {
                Some(dif) => ok(dif > 0.0),
                _ => Err(format!("Cannot compare {} and {}.", a, b))
            }
        });

        // >=
        self.define_intrinsic(">=", |_, args| {
            let len = args.len();
            if len != 2 {
                return Err(arity_exact(2, len));
            }

            let (a, b) = (&args[0], &args[1]);
            let cmp = cmp(a, b);
            match cmp {
                Some(dif) => ok(dif >= 0.0),
                _ => Err(format!("Cannot compare {} and {}.", a, b))
            }
        });

        // eq?
        self.define_intrinsic("eq?", |_, args| {
            let len = args.len();
            if len != 2 {
                return Err(arity_exact(2, len));
            }
            
            let (a, b) = (&args[0], &args[1]);
            ok(a == b)
        });

        // or
        self.define_intrinsic("or", |_, args| {
            use Value::*;
            let len = args.len();
            if len != 2 {
                Err(arity_exact(2, len))
            } else {
                match (&args[0], &args[1]) {
                    (&Bool(a), &Bool(b)) => ok(a || b),
                    _ => Err(format!("\"or\" may only be used on bool values."))
                }
            }
        });

        // and
        self.define_intrinsic("and", |_, args| {
            use Value::*;
            let len = args.len();
            if len != 2 {
                Err(arity_exact(2, len))
            } else {
                match (&args[0], &args[1]) {
                    (&Bool(a), &Bool(b)) => ok(a && b),
                    _ => Err(format!("\"and\" may only be used on bool values."))
                }
            }
        });

        // apply
        self.define_intrinsic("apply", |env, args| {
            use Value::*;
            let len = args.len();
            if len != 2 {
                Err(arity_exact(2, len))
            } else {
                let (func, args) = (&args[0], &args[1]);
                match (func, args) {
                    (&Func(_, _), &List(ref list)) => eval_func(func, &list, env),
                    (&Intrinsic(func), &List(ref list)) => func(env, list),
                    _ => Err(format!("Contract not satisfied: {} {}.", func, args))
                }
            }
        });

        // not
        self.define_intrinsic("not", |_, args| {
            use Value::*;
            let len = args.len();
            if len != 1 {
                Err(arity_exact(1, len))
            } else {
                let arg = &args[0];
                match *arg {
                    Bool(b) => ok(!b),
                    _ => Err(format!("{} is not a bool.", arg))
                }
            }
        });
    }
}

/// Compare the two specified values. If they are numbers, their difference
/// is returned. Otherwise, `None` is returned.
fn cmp(a: &Value, b: &Value) -> Option<f64> {
    use self::Value::*;
    match (a, b) {
        (&Num(a), &Num(b)) => Some(a - b),
        _ => None
    }
}