use interpreter::*;
use sexpr::SExpr;
use errors::*;
use values::*;
use environment::Environment;
use std::collections::HashMap;

/// The name of the lisp interpreter.
const NAME: &'static str = env!("CARGO_PKG_NAME");

/// The version of the lisp interpreter.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// All reserved words that may not be used as identifiers.
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
    fn define_intrinsic<S>(&mut self, ident: S, f: Intrinsic)
        where S: Into<String>;

    fn define_macro<S>(&mut self, ident: S, f: Macro)
        where S: Into<String>;

    fn init_intrinsics(&mut self);
}

impl Intrinsics for Environment {
    fn define_intrinsic<S>(&mut self, ident: S, f: Intrinsic)
        where S: Into<String>
    {
        self.define(ident, Value::Intrinsic(f));
    }

    fn define_macro<S>(&mut self, ident: S, f: Macro)
        where S: Into<String>
    {
        self.define(ident, Value::Macro(f));
    }

    fn init_intrinsics(&mut self) {
        use values::Value::*;

        // Constants
        self.define("empty", nil());

        let infinity = ::std::f64::INFINITY;
        self.define("infinity", Num(infinity));
        self.define("-infinity", Num(-infinity));

        self.define("lisp-version", Str(VERSION.to_string()));
        self.define("lisp-name", Str(NAME.to_string()));

        // Macros
        self.define_macro("define", macros::_define);
        self.define_macro("lambda", macros::_lambda);
        self.define_macro("if", macros::_if);
        self.define_macro("cond", macros::_cond);
        self.define_macro("let", macros::_let);
        self.define_macro("define-struct", macros::_define_struct);

        // Numeric operations
        self.define_intrinsic("+", functions::_add);
        self.define_intrinsic("-", functions::_sub);
        self.define_intrinsic("*", functions::_mul);
        self.define_intrinsic("/", functions::_div);
        self.define_intrinsic("modulo", functions::_modulo);
        self.define_intrinsic("sqrt", functions::_sqrt);
        self.define_intrinsic("pow", functions::_pow);

        // Type checking functions
        self.define_intrinsic("num?", functions::_is_num);
        self.define_intrinsic("bool?", functions::_is_bool);
        self.define_intrinsic("str?", functions::_is_str);
        self.define_intrinsic("cons?", functions::_is_cons);
        self.define_intrinsic("lambda?", functions::_is_lambda);
        self.define_intrinsic("symbol?", functions::_is_symbol);

        // List functions
        self.define_intrinsic("list", functions::_list);
        self.define_intrinsic("cons", functions::_cons);
        self.define_intrinsic("car", functions::_car);
        self.define_intrinsic("cdr", functions::_cdr);
        self.define_intrinsic("len", functions::_len);
        self.define_intrinsic("nth", functions::_nth);

        // Comparison operations
        self.define_intrinsic("<", functions::_is_l);
        self.define_intrinsic("<=", functions::_is_le);
        self.define_intrinsic(">", functions::_is_g);
        self.define_intrinsic(">=", functions::_is_ge);
        self.define_intrinsic("eq?", functions::_is_eq);

        // Logical operations
        self.define_intrinsic("or", functions::_or);
        self.define_intrinsic("and", functions::_and);
        self.define_intrinsic("not", functions::_not);

        // Other
        self.define_intrinsic("exit", functions::_exit);
        self.define_intrinsic("begin", functions::_begin);
        self.define_intrinsic("print", functions::_print);
        self.define_intrinsic("apply", functions::_apply);
        self.define_intrinsic("concat", functions::_concat);
        self.define_intrinsic("eval", functions::_eval);
    }
}

mod macros {
    // Imports
    use super::*;
    use SExpr::*;

    /// Represents the output of a function.
    type Output = Result<Value, String>;

    /// Represents a mutable reference to an environment.
    type Env<'a> = &'a mut Environment;

    /// Represents a slice containing the arguments passed to a function.
    type Exprs<'a> = &'a [SExpr];

    /// `(define ident value)`
    /// 
    /// `(define (func-name param1 ...) body)`
    pub fn _define(env: Env, exprs: Exprs) -> Output {
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
    }

    /// `(lambda [param1 ...] body)
    pub fn _lambda(_: Env, exprs: Exprs) -> Output {
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
                                (&Ident(ref s), expr) => {
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
                (&Ident(ref name), &List(ref vals)) => {
                    let len = vals.len();
                    if len < 1 {
                        Err(arity_exact(1, len))
                    } else {

                        let mut fields: Vec<String> = Vec::with_capacity(len);

                        // Check that all values are identifiers
                        for value in vals.iter() {
                            match value {
                                &Ident(ref ident) => fields.push(ident.clone()),
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
                                if let &SExpr::Ident(ref ident) = name {
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
                                    if let &SExpr::Ident(ref accessor) = accessor {
                                        let hyphen_index = accessor.rfind('-');
                                        if let Some(i) = hyphen_index {
                                            let field_name = &accessor[i + 1..];
                                            let struct_expr = &args[0];
                                            let struct_expr = struct_expr.eval(env)?;
                                            if let Value::Struct(_, ref values) = struct_expr {
                                                let value = values.get(field_name)
                                                    .unwrap()
                                                    .clone();
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
                            if let &SExpr::Ident(ref name) = name {
                                // Name after "make-"
                                let name = &name[5..];
                                let field_names: Vec<String>;
                                {
                                    let fields = env.get_struct(name);

                                    match fields {
                                        Some((_, fields)) => {
                                            field_names = fields.to_vec();
                                        },
                                        _ => return err("Could load struct fields names.")
                                    }
                                }

                                let len = params.len();
                                let expected = field_names.len();

                                if len != expected {
                                    return Err(arity_exact(expected, len));
                                }

                                let mut values = HashMap::<String, Value>::new();
                                for i in 0..expected {
                                    let param = &params[i];
                                    let field = &field_names[i];
                                    let value = param.eval(env)?;
                                    values.insert(field.to_string(), value);
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
}

mod functions {
    // Imports
    use super::*;
    use Value::*;

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
            &Symbol(_) => ok(true),
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
            &Func(_, _) => ok(true),
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
            (&Func(_, _), &List(ref list)) => eval_func(func, &list, env),
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
    fn check_arity(expected: usize, found: usize) -> Output {
        if found != expected {
            Err(arity_exact(expected, found))
        } else {
            ok(nil())
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
}