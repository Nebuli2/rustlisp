// Imports
use super::*;
use SExpr::*;

/// Represents the output of a function.
type Output = Result<Value>;

/// Represents a mutable reference to an environment.
type Env<'a> = &'a mut Environment;

/// Represents a slice containing the arguments passed to a function.
type Exprs<'a> = &'a [SExpr];

/// `(define ident value)`
/// 
/// `(define (func-name param1 ...) body)`
pub fn _define(env: Env, exprs: Exprs) -> Output {
    let len = exprs.len();
    if len > 2 {
        let (ident, val) = (&exprs[1], &exprs[2]);
        match ident {
            // Define variable
            &Ident(ref s, _) => {
                if len == 3 {
                    if RESERVED_WORDS.contains(&s.as_str()) {
                        Err(reserved_word(s))
                    } else {
                        let val = val.eval(env)?;
                        env.define(s.clone(), val);
                        ok(nil())
                    }
                } else {
                    err(arity_exact(2, len - 1))
                }
            },

            // Define function
            // Converts
            //  (define (func-name param1 ...) statements ...)
            // Into:
            //  (define func-name (lambda (param1 ...) (begin statements ...)))
            // If more than 3 args are passed in the original define, wrap last ones in a "begin".
            &List(ref vals) => {
                let vals_len = vals.len();
                if vals_len == 0 {
                    Err(format!("Cannot redefine empty list."))
                } else {
                    let ident = (&vals[0]).clone();
                    let params: Vec<_> = (&vals[1..]).iter()
                        .map(|expr| expr.clone())
                        .collect();

                    let body = if len > 3 {
                        let mut vec = Vec::<SExpr>::with_capacity(vals_len - 1);
                        vec.push(Ident("begin".to_string(), false));

                        let statements = (&exprs[2..]).iter()
                            .map(|expr| expr.clone());
                        vec.extend(statements);
                        List(vec)
                    } else {
                        val.clone()
                    };

                    let define = List(vec![
                        Ident("define".to_string(), false),
                        ident, 
                        List(vec![
                            Ident("lambda".to_string(), false),
                            List(params),
                            body
                        ])
                    ]);

                    define.eval(env)                    
                }
            },
            _ => Err(not_an_identifier(ident))
        }
    } else {
        Err(arity_at_least(2, len - 1))
    }
}

/// `(lambda [param1 ...] body)
pub fn _lambda(_: Env, exprs: Exprs) -> Output {
    let len = exprs.len();
    if len != 3 {
        return Err(arity_exact(2, len - 1));
    }

    let (params, body) = (&exprs[1], &exprs[2]);
    match params {
        &List(ref params) => {
            let len = params.len();
            let mut names = Vec::<String>::with_capacity(len);
            for (i, param) in params.iter().enumerate() {
                match param {
                    &SExpr::Ident(ref s, variadic) => {
                        if variadic && i != len - 1 {
                            return err("Only the final parameter of a function may be variadic.")
                        }
                        names.push(s.to_string())
                    },
                    _ => return err(not_an_identifier(param))
                }
            }
            let variadic = if len > 0 {
                if let &SExpr::Ident(_, v) = &params[len - 1] {
                    v
                } else {
                    false
                }
            } else {
                false   
            };
            Ok(Value::Func(names, body.clone(), variadic))
        },
        _ => Err(not_a_list(params))
    }
}

/// `(if bool value1 value2)`
/// 
/// If the specified bool is true, the first value is returned. Otherwise,
/// the second value is returned.
pub fn _if(env: Env, exprs: Exprs) -> Output {
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
}

/// `(cond [cond1 value1] ...)`
/// 
/// Steps through the condition expressions. If one of the conditions
/// evaluates to true, its value is returned. Otherwise, the next
/// next expression is checked, etc.
pub fn _cond(env: Env, exprs: Exprs) -> Output {
    let conditions = &exprs[1..];
    env.enter_scope();
    env.define("else", Value::Bool(true));
    for condition in conditions.iter() {
        match condition {
            &List(ref vals) => {
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
}

/// `
/// (let ([ident1 value1]
///       ...)
///     expr)
/// `
pub fn _let(env: Env, exprs: Exprs) -> Output {
    let len = exprs.len() - 1;
    if len != 2 {
        return Err(arity_exact(2, len));
    }

    let args = (&exprs[1], &exprs[2]);
    match args {
        (&List(ref bindings), body) => {
            env.enter_scope();
            for expr in bindings.iter() {
                match *expr {
                    List(ref binding) => {
                        let len = binding.len();
                        if len != 2 {
                            return Err(arity_exact(2, len));
                        }

                        let binding = (&binding[0], &binding[1]);
                        match binding {
                            (&Ident(ref s, _), expr) => {
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
}

/// `(define-struct (struct-name field1 ...)`
pub fn _define_struct(env: Env, exprs: Exprs) -> Output {
    let len = exprs.len() - 1;
    if len != 2 {
        Err(arity_exact(2, len))
    } else {
        let (struct_name, struct_def) = (&exprs[1], &exprs[2]);
        match (struct_name, struct_def) {
            (&Ident(ref name, _), &List(ref vals)) => {
                let len = vals.len();
                if len < 1 {
                    Err(arity_exact(1, len))
                } else {

                    let mut fields: Vec<String> = Vec::with_capacity(len);

                    // Check that all values are identifiers
                    for value in vals.iter() {
                        match value {
                            &Ident(ref ident, _) => fields.push(ident.clone()),
                            _ => return Err(not_an_identifier(value))
                        }
                    }

                    env.add_struct(name.clone(), fields.clone());
                    
                    // Define type checker
                    // ({struct}? val)
                    let format = format!("{}?", &name);
                    env.define_macro(format, |env, exprs| {

                        let len = exprs.len();
                        if len != 2 {
                            err(::errors::arity_exact(1, len - 1))
                        } else {
                            let name = &exprs[0];
                            if let &SExpr::Ident(ref ident, _) = name {
                                let len = ident.len();
                                let struct_name = &ident[..len - 1];

                                // Evaluate argument
                                // let struct_expr = &args[0];
                                // let struct_expr = struct_expr.eval(env)?;
                                let value = &exprs[1];
                                let value = value.eval(env)?;
                                if let Value::Struct(ref name, _) = value {
                                    ok(struct_name == name)
                                } else {
                                    ok(false)
                                }
                            } else {
                                ok(false)
                            }
                        }

                    });

                    // Define field accessors
                    // ({struct}-{field} val)
                    for field in fields.iter() {
                        let accessor_name = format!("{}-{}", &name, field);
                        env.define_macro(accessor_name, |env, exprs| {
                            let accessor = &exprs[0];
                            let args = &exprs[1..];
                            let len = args.len();
                            if len != 1 {
                                err(arity_exact(1, len))
                            } else {
                                if let &SExpr::Ident(ref accessor, _) = accessor {
                                    let hyphen_index = accessor.rfind('-');
                                    if let Some(i) = hyphen_index {
                                        let struct_name = &accessor[..i];
                                        let field_name = &accessor[i + 1..];
                                        let struct_expr = &args[0];
                                        let struct_expr = struct_expr.eval(env)?;
                                        if let Value::Struct(_, ref values) = struct_expr {

                                            // We know that these have been defined, so it is
                                            // safe to unwrap them.
                                            let struct_def = env.get_struct(struct_name).unwrap();
                                            let index = struct_def.index(field_name).unwrap();

                                            let value = &values[index];
                                            let value = value.clone();
                                            ok(value)
                                        } else {
                                            err(format!("{} is not a struct.", struct_expr))
                                        }
                                    } else {
                                        err(format!("{} is not an accessor", accessor))
                                    }
                                } else {
                                    err(not_an_identifier(accessor))
                                }
                            }
                        });
                    }

                    let make = format!("make-{}", &name);

                    // Define constructor function
                    env.define_macro(make, |env, exprs| {
                        let name = &exprs[0];
                        let params = &exprs[1..];
                        if let &SExpr::Ident(ref name, _) = name {
                            // Name after "make-"
                            let name = &name[5..];

                            // We know this has been defined, so it is safe to unwrap.
                            let field_names = env.get_struct(name)
                                .unwrap()
                                .clone();
                            
                            let len = params.len();
                            let expected = field_names.len();

                            if len != expected {
                                return Err(arity_exact(expected, len));
                            }

                            let mut values: Vec<Value> = Vec::with_capacity(expected);
                            for param in params.iter() {
                                values.push(param.eval(env)?);
                            }

                            ok(Value::Struct(name.to_string(), values))
                        } else {
                            err(not_an_identifier(name))
                        }
                    });

                    ok(nil())
                }
            },
            _ => Err(not_a_list(struct_def))
        }
    }
}