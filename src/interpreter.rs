use std::collections::HashMap;
use values::*;
use sexpr::SExpr;
use errors::*;
use environment::Environment;

pub type FuncResult = Result<Value, String>;
pub type Intrinsic = fn(&mut Environment, &[Value]) -> FuncResult;
pub type Macro = fn(&mut Environment, &[SExpr]) -> FuncResult;

pub fn empty() -> Value {
    Value::List(vec![])
}

pub trait Eval {
    fn eval(&self, ctx: &mut Environment) -> Result<Value, String>;
}

impl Eval for SExpr {
    fn eval(&self, ctx: &mut Environment) -> Result<Value, String> {
        match *self {
            // Primitives map directly
            SExpr::Num(n) => Ok(Value::Num(n)),
            SExpr::Bool(b) => Ok(Value::Bool(b)),
            SExpr::Str(ref s) => Ok(Value::Str(s.clone())),

            // Fetch value of identifier in context
            SExpr::Ident(ref s) => {
                if let Some(val) = ctx.get(s) {
                    Ok(val.clone())
                } else {
                    Err(unbound(s))
                }
            }

            // Evaluate first element of the list, then apply subsequent
            // elements to the first element if it is a function.
            SExpr::List(ref vals) => {
                if vals.is_empty() {
                    Ok(empty())
                } else {
                    let func = vals[0].eval(ctx)?;
                    match func {
                        Value::Func(params, expr) => {

                            ctx.enter_scope();
                            
                            let params_len = params.len();
                            let args_len = vals.len() - 1;

                            if params_len != args_len {
                                return Err(arity_exact(params_len, args_len));
                            }

                            for i in 0..params.len() {
                                let val = &vals[i + 1];
                                let res = val.eval(ctx)?;
                                ctx.define(params[i].clone(), res);
                            }

                            
                            let res = expr.eval(ctx);
                            ctx.exit_scope();
                            res
                        },
                        Value::Intrinsic(ref func) => {
                            let mut args: Vec<Value> = vec![];

                            for arg in &vals[1..] {
                                let eval = arg.eval(ctx)?;
                                args.push(eval);
                            }

                            func(ctx, &args)
                        },
                        Value::Macro(ref func) => {
                            func(ctx, vals)
                        }
                        _ => Err(not_a_function(&func))
                    }
                }
            },

            // Nil evaluates to an empty list
            SExpr::Nil => Ok(empty())
        }
    }
}

pub fn eval_func(
    func: &Value, 
    args: &Vec<Value>, 
    env: &mut Environment
) -> Result<Value, String> {
    match *func {
        Value::Func(ref params, ref body) => {
            env.enter_scope();

            let params_len = params.len();
            let args_len = args.len() - 1;

            if params_len != args_len {
                return Err(arity_exact(params_len, args_len));
            }

            for i in 0..params.len() {
                let val = &args[i + 1];
                env.define(params[i].clone(), val.clone());
            }

                            
            let res = body.eval(env)?;
            env.exit_scope();
            Ok(res)
        },
        _ => Err(not_a_function(func))
    }
}