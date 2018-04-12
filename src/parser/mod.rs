pub mod sexpr;

use std::io::{BufReader, Read};

// Export SExpr.
pub use sexpr::*;

pub struct Parser<R: Read> {
    stack: Vec<char>,
    reader: BufReader<R>,
}

type ParseResult = Result<SExpr, String>;

impl<R: Read> Parser<R> {
    pub fn new(reader: BufReader<R>) -> Self {
        Parser {
            stack: vec![],
            reader,
        }
    }

    pub fn parse_all(&mut self) -> Result<Vec<SExpr>, String> {
        let mut results: Vec<SExpr> = vec![];

        loop {
            match self.parse() {
                Ok(expr) => results.push(expr),
                Err(why) => {
                    if why == "EOF" {
                        break;
                    } else {
                        return Err(why);
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
                None => break,
            }
        }
    }

    /// Produces the next expression from the reader, or an error if one is not
    /// found.
    pub fn parse(&mut self) -> ParseResult {
        self.skip_whitespace();

        self.next_char()
            .ok_or("EOF".to_string())
            .and_then(|c| match c {
                // Comment
                ';' => {
                    self.skip_to_linebreak();
                    self.parse()
                }

                // Quote
                '\'' => {
                    let quoted = self.parse()?;
                    let quoted = SExpr::Quote(Box::new(quoted));
                    Ok(quoted)
                }

                // List (parentheses)
                '(' => self.parse_list(')'),

                // List (brackets)
                '[' => self.parse_list(']'),

                // String
                '"' => self.parse_str(),

                // Atom
                _ => {
                    self.undo_char(c);
                    self.parse_atom()
                }
            })
    }

    /// Attempts to read the next atom in the `Parser`'s reader into an
    /// `Option<String>`. An atom is defined as being any expression other than
    /// a list.
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

    /// Attempts to parse the next atom in the `Parser`'s reader. An atom is
    /// defined as any expression that is not a list.
    fn parse_atom(&mut self) -> ParseResult {
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
                const ELLIPSIS: &str = "...";
                let variadic = s.ends_with(ELLIPSIS);
                let name = if variadic {
                    s[..s.len() - ELLIPSIS.len()].to_string()
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

    /// Attempts to parse the next string from the `Parser`'s reader.
    fn parse_str(&mut self) -> ParseResult {
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
                            c => return Err(format!("Unknown escape character '\\{}'.", c)),
                        };
                        buf.push(escape);
                    }
                }

                // Otherwise push the character
                Some(c) => buf.push(c),

                // Throw an error if the string is unclosed
                None => return Err("Unexpected EOF before end of string.".to_string()),
            }
        }

        Ok(SExpr::Str(buf))
    }

    /// Attempts to parse the next list from the `Parser`'s reader.
    fn parse_list(&mut self, close: char) -> ParseResult {
        let mut buf: Vec<SExpr> = vec![];

        loop {
            match self.next_char() {
                Some(c) if c.is_whitespace() => (),
                Some(c) if c == close => {
                    break;
                }
                Some(c) => {
                    self.undo_char(c);
                    let exp = self.parse()?;
                    buf.push(exp);
                }
                None => return Err("Unexpected EOF before end of list.".to_string()),
            }
        }

        Ok(SExpr::List(buf))
    }

    /// Attempts to produce the next `char` in the `Parser`'s reader. If the
    /// reader does not contains another `char`, `None` is returned instead.
    fn next_char(&mut self) -> Option<char> {
        if self.stack.is_empty() {
            let mut buf: [u8; 1] = [0];
            match self.reader.read(&mut buf) {
                Ok(n) => match n {
                    1 => (), // Read one char as expected
                    _ => return None,
                },
                Err(_) => return None,
            }
            let ch = buf[0] as char;
            Some(ch)
        } else {
            self.stack.pop()
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.next_char() {
                Some(c) if c.is_whitespace() => (),
                Some(c) => {
                    self.undo_char(c);
                    break;
                }
                None => break,
            }
        }
    }

    /// Undoes the last read `char`.
    fn undo_char(&mut self, c: char) {
        self.stack.push(c);
    }
}

// val

trait ValidParse {
    fn is_valid_atom(&self) -> bool;
    fn is_valid_ident(&self) -> bool;
}

impl ValidParse for char {
    // Determines whether or not the item is a valid beginning to an atom.
    fn is_valid_atom(&self) -> bool {
        match *self {
            '(' | '[' | ')' | ']' => false,
            _ => true,
        }
    }

    /// Determines whether or not the item is valid for use in an identifier.
    fn is_valid_ident(&self) -> bool {
        match *self {
            '-'
            | '_'
            | '+'
            | '/'
            | '*'
            | '%'
            | '>'
            | '<'
            | '='
            | '?'
            | '!'
            | '&'
            | '$'
            | '.'
            | '#'
            | ':'
            | 'λ'
            | 'a'...'z'
            | 'A'...'Z'
            | '0'...'9' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Into<BufReader<char>> for String {
        fn into(self) -> BufReader<char> {}
    }
}
