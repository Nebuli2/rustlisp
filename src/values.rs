use sexpr::*;
use interpreter::*;
use std::fmt;
use std::collections::HashMap;
use environment::Environment;

#[derive(Clone)]
pub enum Value {
    Num(f64),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    Func(Vec<String>, SExpr),
    Intrinsic(Intrinsic),
    Macro(Macro),
    Struct(String, HashMap<String, Value>)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            // num
            &Num(n) => write!(f, "{}", n),

            // #t | #f
            &Bool(b) => if b {
                write!(f, "true")
            } else {
                write!(f, "false")
            },

            // "string"
            &Str(ref s) => write!(f, "\"{}\"", s),

            // '(a b c ...)
            &List(ref exps) => {
                write!(f, "'(")?;
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
            &Func(ref args, ref body) => {
                // Write lambda 
                write!(f, "(lambda (")?;

                // Write args
                if !args.is_empty() {
                    for i in 0..args.len() - 1 {
                        write!(f, "{} ", &args[i])?;
                    }
                    write!(f, "{}", &args[args.len() - 1])?;  
                }

                // Write body
                write!(f, ") {})", body)
            },

            // <Intrinsic>
            &Intrinsic(_) => {
                write!(f, "<Intrinsic>")
            },

            // <Macro>
            &Macro(_) => {
                write!(f, "<Macro>")
            },

            // { a: b, c: d, e: f }
            &Struct(_, ref values) => {
                // Write opening bracket
                write!(f, "{{")?;

                // Write keys, values in format: key: value
                let last = values.len() - 1;
                for (i, (key, value)) in values.iter().enumerate() {
                    write!(f, "{}: {}", key, value)?;
                    if i < last {
                        write!(f, ",")?;
                    }
                }

                // Write closing bracket
                write!(f, "}}")
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
pub fn ok<T>(val: T) -> Result<Value, String> 
    where T: Into<Value> 
{
    Ok(val.into())
}

pub fn err<T>(msg: T) -> Result<Value, String>
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
            _ => false
        }
    }
}