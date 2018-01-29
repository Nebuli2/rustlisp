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
    Struct(String, Vec<Value>)
}

impl From<SExpr> for Value {
    fn from(expr: SExpr) -> Value {
        match expr {
            SExpr::Num(n) => Value::Num(n),
            SExpr::Bool(n) => Value::Bool(n),
            SExpr::Str(s) => Value::Str(s),
            SExpr::Ident(s, v) => Value::Symbol(s, v),
            SExpr::List(vals) => Value::List(  
                vals.into_iter().map(|expr| expr.into()).collect()
            ),
            SExpr::Nil => Value::List(vec![]),
            SExpr::Quote(expr) => (*expr).into()
        }
    }
}

impl Into<SExpr> for Value {
    fn into(self) -> SExpr {
        match self {
            Value::Num(n) => SExpr::Num(n),
            Value::Bool(n) => SExpr::Bool(n),
            Value::Str(s) => SExpr::Str(s),
            Value::Symbol(s, v) => SExpr::Ident(s, v),
            Value::List(vals) => SExpr::List(
                vals.into_iter().map(|expr| expr.into()).collect()
            ),
            Value::Struct(ref name, ref fields) => {
                let mut exprs: Vec<SExpr> = Vec::with_capacity(fields.len() + 1);
                exprs.push(SExpr::Ident(format!("make-{}", name), false));
                for field in fields {
                    exprs.push(field.clone().into());
                }
                SExpr::List(exprs)
            }
            _ => panic!("Evaluating other values is not yet supported.")
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            // num
            &Num(n) => {
                let out = n;
                write!(f, "{}", out)
            },

            // #t | #f
            &Bool(b) => if b {
                let out = "true";
                write!(f, "{}", out)
            } else {
                let out = "false";
                write!(f, "{}", out)
            },

            // "string"
            &Str(ref s) => {
                write!(f, "{}", s)
            },

            // 'symbol
            &Symbol(ref s, v) => {
                write!(f, "{}", s)?;
                if v {
                    write!(f, "...")?;
                }
                Ok(())
            }

            // (a b c ...)
            &List(ref exps) => {
                write!(f, "(")?;
                let len = exps.len();
                if len > 0 {
                    for i in 0..len - 1 {
                        write!(f, "{} ", &exps[i])?;
                    }
                    write!(f, "{}", &exps[len - 1])?;
                    
                }
                write!(f, ")")
            },

            // (lambda (params ...) body)
            &Func(ref args, ref body, variadic) => {
                // Write lambda 
                write!(f, "(lambda (")?;

                // Write args
                if !args.is_empty() {
                    for i in 0..args.len() - 1 {
                        write!(f, "{} ", &args[i])?;
                    }
                    write!(f, "{}", &args[args.len() - 1])?;  
                    if variadic {
                        write!(f, "...")?;
                    }
                }

                // Write body
                write!(f, ") {})", body)
            },

            // <function>
            &Intrinsic(_) => {
                write!(f, "<function>")
            },

            // <procedure>
            &Macro(_) => {
                write!(f, "<procedure>")
            },

            // (make-{struct} {field1} ...)
            &Struct(ref name, ref values) => {
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
    fn into(self) -> Value {
        Value::Num(self)
    }
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Bool(self)
    }
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::Str(self)
    }
}

/// Wrap the specified value in an `Ok`.
pub fn ok<T>(val: T) -> ::std::result::Result<Value, String> 
    where T: Into<Value> 
{
    Ok(val.into())
}

pub fn err<T>(msg: T) -> Result<Value>
    where T: Into<String>
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
            (&Symbol(ref a, a_vec), &Symbol(ref b, b_vec)) => {
                a == b && a_vec == b_vec
            },
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
            },
            (&Struct(ref a_ident, ref a_fields), &Struct(ref b_ident, ref b_fields)) => {
                if a_ident == b_ident {
                    for (i, a_value) in a_fields.iter().enumerate() {
                        let b_value = &b_fields[i];
                        if a_value != b_value {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }
}