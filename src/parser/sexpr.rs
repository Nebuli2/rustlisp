use self::SExpr::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SExpr {
    Str(String),
    Num(f64),
    Bool(bool),
    Ident(String, bool),
    List(Vec<SExpr>),
    Quote(Box<SExpr>),
    Nil,
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // "string"
            Str(s) => write!(f, "\"{}\"", s),

            // num
            Num(n) => write!(f, "{}", n),

            // #t | #f
            Bool(b) => if *b {
                write!(f, "true")
            } else {
                write!(f, "false")
            },

            // ident
            Ident(s, variadic) => {
                write!(f, "{}", s)?;
                if *variadic {
                    write!(f, "...")?;
                }
                Ok(())
            }

            // (a b c ...)
            List(exps) => {
                write!(f, "(")?;
                let len = exps.len();
                if len > 0 {
                    for i in 0..len - 1 {
                        write!(f, "{} ", &exps[i])?;
                    }
                    write!(f, "{}", &exps[len - 1])?;
                }
                write!(f, ")")
            }

            // 'quote
            Quote(ref expr) => write!(f, "'{}", expr),

            // '()
            Nil => write!(f, "'()"),
        }
    }
}
