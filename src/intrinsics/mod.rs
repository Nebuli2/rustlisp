use interpreter::*;
use parser::SExpr;
use errors::*;
use interpreter::Environment;

mod macros;
pub mod functions;

/// The name of the lisp interpreter.
const NAME: &str = env!("CARGO_PKG_NAME");

/// The version of the lisp interpreter.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// All reserved words that may not be used as identifiers.
const RESERVED_WORDS: [&str; 7] = [
    "define",
    "define-struct",
    "begin",
    "cond",
    "else",
    "if",
    "let",
];

fn nil() -> Value {
    Value::List(vec![])
}

pub trait Intrinsics {
    fn define_intrinsic<S>(&mut self, S, Intrinsic)
    where
        S: Into<String>;

    fn define_macro<S>(&mut self, S, Macro)
    where
        S: Into<String>;

    fn init_intrinsics(&mut self);
}

impl Intrinsics for Environment {
    fn define_intrinsic<S>(&mut self, ident: S, f: Intrinsic)
    where
        S: Into<String>,
    {
        self.define(ident, Value::Intrinsic(f));
    }

    fn define_macro<S>(&mut self, ident: S, f: Macro)
    where
        S: Into<String>,
    {
        self.define(ident, Value::Macro(f));
    }

    fn init_intrinsics(&mut self) {
        use self::Value::*;

        // Constants
        self.define("empty", nil());

        let infinity = ::std::f64::INFINITY;
        self.define("math/infinity", Num(infinity));
        self.define("math/-infinity", Num(-infinity));

        let pi = ::std::f64::consts::PI;
        let e = ::std::f64::consts::E;
        self.define("math/pi", Num(pi));
        self.define("math/e", Num(e));

        self.define("env/lisp-version", Str(VERSION.to_string()));
        self.define("env/lisp-name", Str(NAME.to_string()));

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
        self.define_intrinsic("log", functions::_log);
        self.define_intrinsic("fibonacci", functions::_fib_rust);

        // Type checking functions
        functions::load_checks(self);

        // List functions
        // self.define_intrinsic("list", functions::_list);
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
        self.define_intrinsic("println", functions::_println);
        self.define_intrinsic("apply", functions::_apply);
        self.define_intrinsic("concat", functions::_concat);
        self.define_intrinsic("eval", functions::_eval);

        self.define_intrinsic("format", functions::_format);
        self.define_intrinsic("read-line", functions::_read_line);
        self.define_intrinsic("parse", functions::_parse);

        self.define_intrinsic("import", functions::_import);
        self.define_intrinsic("read-file", functions::_read_file);
        self.define_intrinsic("write-file", functions::_write_file);

        functions::load_trig_fns(self);
    }
}
