use interpreter::Value;
use err::RLError as Error;
use parser::SExpr;

pub type Result<T> = ::std::result::Result<T, Error>;

pub fn arity_at_least(expected: usize, found: usize) -> Error {
    format!("Expected at least {} arg(s), found {}.", expected, found).into()
}

pub fn arity_at_most(expected: usize, found: usize) -> Error {
    format!("Expected at most {} arg(s), found {}.", expected, found).into()
}

pub fn arity_exact(expected: usize, found: usize) -> Error {
    format!("Expected {} arg(s), found {}.", expected, found).into()
}

pub fn unbound(ident: &str) -> Error {
    format!("Variable {} is unbound.", ident).into()
}

pub fn not_a_function(val: &Value) -> Error {
    format!("{} is not a function.", val).into()
}

pub fn not_a_number(val: &Value) -> Error {
    format!("{} is not a number.", val).into()
}

pub fn not_an_identifier(val: &SExpr) -> Error {
    format!("{} is not an identifier.", val).into()
}

pub fn not_a_list(val: &SExpr) -> Error {
    format!("{} is not a list.", val).into()
}

pub fn not_a_bool(val: &SExpr) -> Error {
    format!("{} is not a bool.", val).into()
}

pub fn reserved_word(val: &str) -> Error {
    format!("\"{}\" is a reserved word.", val).into()
}
