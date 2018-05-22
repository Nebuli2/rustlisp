use super::*;
use interpreter::SExpr;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Num(f64),
    Bool(bool),
    Str(String),
    Symbol(String, bool),
    List(Vec<Value>),
    Func(Vec<String>, SExpr, bool),
    Intrinsic(Intrinsic),
    Macro(Macro),
    Struct(String, Vec<Value>),
}

impl From<SExpr> for Value {
    /// Converts the specified `SExpr` into a `Value`.
    fn from(expr: SExpr) -> Value {
        match expr {
            SExpr::Num(n) => Value::Num(n),
            SExpr::Bool(n) => Value::Bool(n),
            SExpr::Str(s) => Value::Str(s),
            SExpr::Ident(s, v) => Value::Symbol(s, v),
            SExpr::List(vals) => {
                Value::List(vals.into_iter().map(|expr| Value::from(expr)).collect())
            }
            SExpr::Nil => Value::List(vec![]),
            SExpr::Quote(expr) => (*expr).into(),
        }
    }
}

impl Into<SExpr> for Value {
    /// Converts the `Value` into an `SExpr`.
    fn into(self) -> SExpr {
        match self {
            Value::Num(n) => SExpr::Num(n),
            Value::Bool(n) => SExpr::Bool(n),
            Value::Str(s) => SExpr::Str(s),
            Value::Symbol(s, v) => SExpr::Ident(s, v),
            Value::List(vals) => SExpr::List(vals.into_iter().map(|expr| expr.into()).collect()),
            Value::Struct(ref name, ref fields) => {
                let mut exprs: Vec<SExpr> = Vec::with_capacity(fields.len() + 1);
                exprs.push(SExpr::Ident(format!("make-{}", name), false));
                for field in fields {
                    exprs.push(field.clone().into());
                }
                SExpr::List(exprs)
            }
            _ => panic!("Evaluating other values is not yet supported."),
        }
    }
}

impl fmt::Display for Value {
    /// Displays the `Value` in a human-readable format based on the type:
    /// * *num:* Displays as is.
    /// * *bool:* Displays as either `true` or `false`.
    /// * *str:* Displays the string as is.
    /// * *symbol:* Displays the symbol as is.
    /// * *list:* Displays the list in the form: (a b c ...)
    /// * *lambda:* Displays the lambda in the form: (lambda (params ...) body)
    /// * *struct:* Displays the struct in the form: (make-{struct} fields ...)
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        use color::*;
        match self {
            // num
            Num(n) => {
                let out = n.to_string();
                write!(f, "{}", number(out))
            }

            // #t | #f
            Bool(b) => if *b {
                let out = "true";
                write!(f, "{}", boolean(out))
            } else {
                let out = "false";
                write!(f, "{}", boolean(out))
            },

            // "string"
            Str(s) => {
                let out = format!("\"{}\"", s);
                write!(f, "{}", string(out))
            }

            // 'symbol
            Symbol(s, v) => {
                write!(f, "{}", s)?;
                if *v {
                    write!(f, "...")?;
                }
                Ok(())
            }

            // (a b c ...)
            List(exps) => {
                if !exps.is_empty() {
                    write!(f, "(")?;
                    let len = exps.len();
                    if len > 0 {
                        for i in 0..len - 1 {
                            write!(f, "{} ", &exps[i])?;
                        }
                        write!(f, "{}", &exps[len - 1])?;
                    }
                    write!(f, ")")
                } else {
                    Ok(())
                }
            }

            // (lambda (params ...) body)
            Func(args, body, variadic) => {
                // Write lambda
                write!(f, "(lambda (")?;

                // Write args
                if !args.is_empty() {
                    for i in 0..args.len() - 1 {
                        write!(f, "{} ", &args[i])?;
                    }
                    write!(f, "{}", &args[args.len() - 1])?;
                    if *variadic {
                        write!(f, "...")?;
                    }
                }

                // Write body
                write!(f, ") {})", body)
            }

            // <function>
            Intrinsic(_) => write!(f, "<function>"),

            // <procedure>
            Macro(_) => write!(f, "<procedure>"),

            // (make-{struct} {field1} ...)
            Struct(name, values) => {
                // Write opening bracket
                write!(f, "(make-{}", name)?;

                // Write keys, values in format: key: value
                for value in values.iter() {
                    write!(f, " {}", value)?;
                }

                // Write closing bracket
                write!(f, ")")
            }
        }
    }
}

impl Into<Value> for f64 {
    /// Converts the specified `f64` into a num `Value`.
    fn into(self) -> Value {
        Value::Num(self)
    }
}

impl Into<Value> for bool {
    /// Converts the specified `bool` into a bool `Value`.
    fn into(self) -> Value {
        Value::Bool(self)
    }
}

impl Into<Value> for String {
    /// Converts the specified `String` into a str `Value`.
    fn into(self) -> Value {
        Value::Str(self)
    }
}

use err::RLError;
use errors::Result;

/// Wrap the specified value in an `Ok`.
pub fn ok<T>(val: T) -> Result<Value>
where
    T: Into<Value>,
{
    Ok(val.into())
}

impl Into<Value> for Vec<Value> {
    fn into(self) -> Value {
        Value::List(self)
    }
}

pub fn err<T>(msg: T) -> Result<Value>
where
    T: Into<RLError>,
{
    Err(msg.into())
}

impl PartialEq for Value {
    /// Compare the two values to one another for equality.
    fn eq(&self, other: &Value) -> bool {
        use self::Value::*;
        match (self, other) {
            (&Num(a), &Num(b)) => a == b,
            (&Bool(a), &Bool(b)) => a == b,
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Symbol(ref a, a_vec), &Symbol(ref b, b_vec)) => a == b && a_vec == b_vec,
            (&List(ref a), &List(ref b)) => {
                if a.len() == b.len() {
                    for i in 0..a.len() {
                        let (a_val, b_val) = (&a[i], &b[i]);
                        if a_val != b_val {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            (&Struct(ref a_type, ref a_fields), &Struct(ref b_type, ref b_fields)) => {
                let a_len = a_fields.len();
                let b_len = b_fields.len();
                if a_type == b_type && a_len == b_len {
                    for i in 0..a_len {
                        let (a, b) = (&a_fields[i], &b_fields[i]);
                        if a != b {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
