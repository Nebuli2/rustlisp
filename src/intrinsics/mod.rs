use interpreter::*;
use sexpr::SExpr;
use errors::*;
use values::*;
use environment::*;

mod macros;
pub mod functions;

/// The name of the lisp interpreter.
const NAME: &'static str = env!("CARGO_PKG_NAME");

/// The version of the lisp interpreter.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// All reserved words that may not be used as identifiers.
const RESERVED_WORDS: [&'static str; 7] = [
    "define",
    "define-struct",
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
    fn define_intrinsic<S>(&mut self, S, Intrinsic) where S: Into<String>;

    fn define_macro<S>(&mut self, S, Macro) where S: Into<String>;

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
        self.define_intrinsic("append", functions::_append);

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

        self.define_intrinsic("format", functions::_format);
    }
}