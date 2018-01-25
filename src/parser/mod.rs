pub mod sexpr;

use std::io::{
    BufReader, 
    Read
};
pub use sexpr::*;

pub struct Parser {
    stack: Vec<char>
}

type Parse = Result<SExpr, String>;

impl Parser {

    pub fn new() -> Parser {
        Parser {
            stack: vec![]
        }
    }

    // pub fn parse_from_str(&mut self, s: &str) -> Parse {
    //     let mut buf = BufReader::new(s.as_bytes());
    //     self.parse(&mut buf)
    // }

    pub fn parse_all<R: Read>(&mut self, r: &mut BufReader<R>) -> Result<Vec<SExpr>, String> {
        let mut results: Vec<SExpr> = vec![];

        loop {
            match self.parse(r) {
                Ok(expr) => results.push(expr),
                Err(why) => {
                    if why == "EOF" {
                        break;
                    } else {
                        return Err(why)
                    }
                }
            }
        }

        Ok(results)
    }

    /// Skips the parser forward until a linebreak is reached.
    fn skip_to_linebreak<R: Read>(&mut self, r: &mut BufReader<R>) {
        loop {
            match self.next_char(r) {
                Some(c) if c == '\n' => break,
                Some(_) => continue,
                None => break
            }
        }
    }

    /// Produces the next expression from the reader, or an error if one is not
    /// found.
    pub fn parse<R: Read>(&mut self, r: &mut BufReader<R>) -> Parse {
        loop {
            match self.next_char(r) {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => match c {
                    ';' => {
                        self.skip_to_linebreak(r);
                        return self.parse(r)
                    }
                    '\'' => {
                        let quoted = self.parse(r)?;
                        let quoted = SExpr::Quote(Box::new(quoted));
                        return Ok(quoted)
                    },
                    '(' => return self.parse_list(r, ')'),
                    '[' => return self.parse_list(r, ']'),
                    '"' => return self.parse_str(r),
                    c => {
                        self.undo_char(c);
                        return self.parse_atom(r);
                    }
                },
                None => return Err("EOF".to_string())
            }
        }
    }

    fn read_atom<R: Read>(&mut self, r: &mut BufReader<R>) -> Option<String> {
        let mut buf = String::new();

        while let Some(c) = self.next_char(r) {
            if !c.is_valid_atom() {
                self.undo_char(c);
                break;
            }

            if c.is_whitespace() {
                self.undo_char(c);
                break;
            } else {
                buf.push(c);
            }
        }

        if buf.is_empty() {
            None
        } else {
            Some(buf)
        }
    }

    fn parse_atom<R: Read>(&mut self, r: &mut BufReader<R>) -> Parse {
        let atom = self.read_atom(r);
        if let Some(s) = atom {
            // Check true
            if s == "#t" || s == "true" {
                return Ok(SExpr::Bool(true));
            }

            // Check false
            if s == "#f" || s == "false" {
                return Ok(SExpr::Bool(false));
            }

            // Check num
            if let Ok(num) = s.parse::<f64>() {
                return Ok(SExpr::Num(num));
            }

            // Check if valid identifier
            if s.chars().all(|c| c.is_valid_ident()) {
                let ellipsis = "...";
                let variadic = s.ends_with(ellipsis);
                let name = if variadic {
                    s[..s.len() - ellipsis.len()].to_string()
                } else {
                    s
                };
                if name.is_empty() {
                    Err("Empty identifier.".to_string())
                } else {
                    Ok(SExpr::Ident(name, variadic))
                }
            } else {
                Err(format!("Invalid identifier {}.", s))
            }
        } else {
            Err("No atom.".to_string())
        }
    }

    fn parse_str<R: Read>(&mut self, r: &mut BufReader<R>) -> Parse {
        let mut buf = String::new();

        // Read chars until closing quotes. If the end of the buffer is reached
        // before a closing quote has been reached, None is returned.
        loop {
            match self.next_char(r) {
                Some(c) if c == '"' => break,
                Some(c) => buf.push(c),
                None => return Err("Unexpected EOF before end of string.".to_string())
            }
        }

        Ok(SExpr::Str(buf))
    }

    fn parse_list<R: Read>(&mut self, r: &mut BufReader<R>, close: char) -> Parse {
        let mut buf: Vec<SExpr> = vec![];

        loop {
            match self.next_char(r) {
                Some(c) if c.is_whitespace() => continue,
                Some(c) if c == close => {
                    break;
                },
                Some(c) => {
                    self.undo_char(c);
                    let exp = self.parse(r)?;
                    buf.push(exp);
                },
                None => return Err("Unexpected EOF before end of list.".to_string())
            }
        }

        Ok(SExpr::List(buf))
    }

    fn next_char<R: Read>(&mut self, r: &mut BufReader<R>) -> Option<char> {
        if self.stack.is_empty() {
            let mut buf: [u8; 1] = [0];
            match r.read(&mut buf) {
                Ok(n) => match n {
                    1 => (), // Read one char as expected
                    _ => return None
                },
                Err(_) => return None
            }
            let ch = buf[0] as char;
            Some(ch)
        } else {
            self.stack.pop()
        }
    }

    fn undo_char(&mut self, c: char) {
        self.stack.push(c);
    }
}

trait ValidAtom {
    fn is_valid_atom(&self) -> bool;
}

trait ValidIdent {
    fn is_valid_ident(&self) -> bool;
}

impl ValidAtom for char {
    fn is_valid_atom(&self) -> bool {
        match *self {
            '(' | '[' | ')' | ']' => false,
            _ => true
        }
    }
}

impl ValidIdent for char {
    fn is_valid_ident(&self) -> bool {
        match *self {
            '-' | '_' | '+' | '/' | '*' |
            '%' | '>' | '<' | '=' | '?' |
            '!' | '&' | '$' | '.' | '#' |
            ':' | 'λ' |
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => true,
            _ => false
        }
    }
}