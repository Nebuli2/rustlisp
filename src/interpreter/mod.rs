mod environment;
mod value;

pub use self::environment::*;
pub use self::value::*;

use parser::SExpr;
use errors::*;

pub type FuncResult = Result<Value>;
pub type Intrinsic = fn(&mut Environment, &[Value]) -> FuncResult;
pub type Macro = fn(&mut Environment, &[SExpr]) -> FuncResult;

pub fn empty() -> Value {
    Value::List(vec![])
}

pub trait Eval {
    fn eval(&self, &mut Environment) -> Result<Value>;
}

const SUPER: &str = "#super:";
const SUPER_LEN: usize = 7;

impl Eval for SExpr {
    fn eval(&self, env: &mut Environment) -> Result<Value> {
        match *self {
            // Primitives map directly
            SExpr::Num(n) => Ok(Value::Num(n)),
            SExpr::Bool(b) => Ok(Value::Bool(b)),
            SExpr::Str(ref s) => Ok(Value::Str(s.clone())),

            // Fetch value of identifier in context
            SExpr::Ident(ref s, _) => {
                // Previous scope if identifier begins with "super:"
                let index = s.find(SUPER);
                let contains_super = match index {
                    Some(_) => true,
                    _ => false,
                };

                let ident = match index {
                    Some(_) => &s[SUPER_LEN..],
                    None => s,
                };

                let res = if contains_super {
                    env.get_super(ident)
                } else {
                    env.get(ident)
                };

                match res {
                    Some(val) => ok(val.clone()),
                    None => err(unbound(ident)),
                }
            }

            // Evaluate first element of the list, then apply subsequent
            // elements to the first element if it is a function.
            SExpr::List(ref vals) => {
                if vals.is_empty() {
                    Ok(empty())
                } else {
                    let func = vals[0].eval(env)?;
                    match func {
                        Value::Func(..) => {
                            let mut args = Vec::<Value>::with_capacity(vals.len() - 1);
                            for param in &vals[1..] {
                                let arg = param.eval(env)?;
                                args.push(arg);
                            }
                            eval_func(&func, &args, env)
                        }
                        Value::Intrinsic(ref func) => {
                            let mut args: Vec<Value> = vec![];

                            for arg in &vals[1..] {
                                let eval = arg.eval(env)?;
                                args.push(eval);
                            }

                            func(env, &args)
                        }
                        Value::Macro(ref func) => func(env, vals),
                        _ => Err(not_a_function(&func)),
                    }
                }
            }

            // Quoted expression
            SExpr::Quote(ref expr) => {
                let r = expr.as_ref().clone();
                Ok(r.into())
            }

            // Nil evaluates to an empty list
            SExpr::Nil => Ok(empty()),
        }
    }
}

/// Attempts to evaluate the specified function, given the specified arguments,
/// in the specified environment.
pub fn eval_func(func: &Value, args: &[Value], env: &mut Environment) -> Result<Value> {
    match func {
        &Value::Func(ref params, ref body, variadic) => {
            env.enter_scope();

            let params_len = params.len();
            let args_len = args.len();

            // Check arity
            if variadic {
                // Variadic parameter does not need to be filled
                if params_len - 1 > args_len {
                    return Err(arity_at_least(params_len - 1, args_len));
                }
            } else if params_len != args_len {
                return Err(arity_exact(params_len, args_len));
            }

            if variadic {
                for i in 0..params_len - 1 {
                    let val = &args[i];
                    env.define(params[i].clone(), val.clone());
                }

                let variadic_arg = if args_len >= params_len {
                    let mut variadic_arg = Vec::<Value>::with_capacity(args_len - params_len + 1);
                    for extra_arg in &args[params_len - 1..] {
                        variadic_arg.push(extra_arg.clone());
                    }
                    variadic_arg
                } else {
                    Vec::new()
                };

                env.define(params[params_len - 1].clone(), Value::List(variadic_arg));
            } else {
                for i in 0..params_len {
                    let val = &args[i];
                    env.define(params[i].clone(), val.clone());
                }
            }

            let res = body.eval(env)?;
            env.exit_scope();
            Ok(res)
        }
        _ => Err(not_a_function(func)),
    }
}
