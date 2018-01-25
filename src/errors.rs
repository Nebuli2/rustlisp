use interpreter::Value;
use parser::SExpr;

pub type Error = String;
pub type Result<T> = ::std::result::Result<T, Error>;

pub fn arity_at_least(expected: usize, found: usize) -> String {
    format!("Expected at least {} arg(s), found {}.", expected, found)
}

pub fn arity_at_most(expected: usize, found: usize) -> String {
    format!("Expected at most {} arg(s), found {}.", expected, found)
}

pub fn arity_exact(expected: usize, found: usize) -> String {
    format!("Expected {} arg(s), found {}.", expected, found)
}

pub fn unbound(ident: &str) -> String {
    format!("Variable {} is unbound.", ident)
}

pub fn not_a_function(val: &Value) -> String {
    format!("{} is not a function.", val)
}

pub fn not_a_number(val: &Value) -> String {
    format!("{} is not a number.", val)
}

pub fn not_an_identifier(val: &SExpr) -> String {
    format!("{} is not an identifier.", val)
}

pub fn not_a_list(val: &SExpr) -> String {
    format!("{} is not a list.", val)
}

pub fn not_a_bool(val: &SExpr) -> String {
    format!("{} is not a bool.", val)
}

pub fn reserved_word(val: &str) -> String {
    format!("\"{}\" is a reserved word.", val)
}

