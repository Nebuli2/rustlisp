pub mod sexpr;

use std::io::{
    BufReader, 
    Read
};
pub use sexpr::*;

pub struct Parser<R: Read> {
    stack: Vec<char>,
    reader: BufReader<R>
}

type Parse = Result<SExpr, String>;

impl<R: Read> Parser<R> {

    pub fn new(reader: BufReader<R>) -> Self {
        Parser {
            stack: vec![],
            reader
        }
    }

    // pub fn parse_from_str(&mut self, s: &str) -> Parse {
    //     let mut buf = BufReader::new(s.as_bytes());
    //     self.parse(&mut buf)
    // }

    pub fn parse_all(&mut self) -> Result<Vec<SExpr>, String> {
        let mut results: Vec<SExpr> = vec![];

        loop {
            match self.parse() {
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
    fn skip_to_linebreak(&mut self) {
        loop {
            match self.next_char() {
                Some(c) if c == '\n' => break,
                Some(_) => continue,
                None => break
            }
        }
    }

    /// Produces the next expression from the reader, or an error if one is not
    /// found.
    pub fn parse(&mut self) -> Parse {
        loop {
            match self.next_char() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) => match c {
                    ';' => {
                        self.skip_to_linebreak();
                        return self.parse()
                    }
                    '\'' => {
                        let quoted = self.parse()?;
                        let quoted = SExpr::Quote(Box::new(quoted));
                        return Ok(quoted)
                    },
                    '(' => return self.parse_list(')'),
                    '[' => return self.parse_list(']'),
                    '"' => return self.parse_str(),
                    c => {
                        self.undo_char(c);
                        return self.parse_atom();
                    }
                },
                None => return Err("EOF".to_string())
            }
        }
    }

    fn read_atom(&mut self) -> Option<String> {
        let mut buf = String::new();

        while let Some(c) = self.next_char() {
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

    fn parse_atom(&mut self) -> Parse {
        let atom = self.read_atom();
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

    fn parse_str(&mut self) -> Parse {
        let mut buf = String::new();

        // Read chars until closing quotes. If the end of the buffer is reached
        // before a closing quote has been reached, None is returned.
        loop {
            match self.next_char() {
                // Stop if a closing quote is reached
                Some(c) if c == '"' => break,

                // Push next character if it is escaped
                Some(c) if c == '\\' => {
                    if let Some(c) = self.next_char() {
                        let escape = match c {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '\"' => '\"',
                            '0' => '\0',
                            '\\' => '\\',
                            c => return Err(format!("Unknown escape character '\\{}'.", c))
                        };
                        buf.push(escape);
                    }
                },

                // Otherwise push the character
                Some(c) => buf.push(c),

                // Throw an error if the string is unclosed
                None => return Err("Unexpected EOF before end of string.".to_string())
            }
        }

        Ok(SExpr::Str(buf))
    }

    fn parse_list(&mut self, close: char) -> Parse {
        let mut buf: Vec<SExpr> = vec![];

        loop {
            match self.next_char() {
                Some(c) if c.is_whitespace() => continue,
                Some(c) if c == close => {
                    break;
                },
                Some(c) => {
                    self.undo_char(c);
                    let exp = self.parse()?;
                    buf.push(exp);
                },
                None => return Err("Unexpected EOF before end of list.".to_string())
            }
        }

        Ok(SExpr::List(buf))
    }

    fn next_char(&mut self) -> Option<char> {
        if self.stack.is_empty() {
            let mut buf: [u8; 1] = [0];
            match self.reader.read(&mut buf) {
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

trait ValidParse {
    fn is_valid_atom(&self) -> bool;
    fn is_valid_ident(&self) -> bool;
}

impl ValidParse for char {
    fn is_valid_atom(&self) -> bool {
        match *self {
            '(' | '[' | ')' | ']' => false,
            _ => true
        }
    }

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