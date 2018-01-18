use std::collections::HashMap;
use values::Value;

type Scope = HashMap<String, Value>;

pub struct Environment {
    base: Scope,
    stack: Vec<Scope>,
    structs: HashMap<String, Vec<String>>
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            base: HashMap::new(),
            stack: vec![],
            structs: HashMap::new()
        };
        env.enter_scope();
        env
    }

    pub fn structs(&self) -> &HashMap<String, Vec<String>> {
        &self.structs
    }

    pub fn structs_mut(&mut self) -> &mut HashMap<String, Vec<String>> {
        &mut self.structs
    }

    pub fn add_struct<S: Into<String>>(&mut self, name: S, fields: Vec<String>) {
        self.structs_mut().insert(name.into(), fields);
    }

    pub fn get_struct<S: Into<String>>(&self, name: S) -> Option<(String, &[String])> {
        let name = name.into();
        let fields = self.structs().get(&name);
        match fields {
            Some(ref fields) => Some((name, fields)),
            _ => None
        }
    }

    pub fn prev_scope(&self) -> &Scope {
        let len = self.stack.len();
        if len > 1 { 
            &self.stack[len - 1] 
        } else {
            &self.base
        }
    }

    pub fn cur_scope(&self) -> &Scope {
        let len = self.stack.len();
        &self.stack[len - 1]
    }

    pub fn cur_scope_mut(&mut self) -> &mut Scope {
        let len = self.stack.len();
        &mut self.stack[len - 1]
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop().expect("Attempted to exit nonexistent scope.");
    }

    pub fn define<K>(&mut self, key: K, value: Value) 
        where K: Into<String>
    {
        let scope = self.cur_scope_mut();
        scope.insert(key.into(), value);
    }

    pub fn get<K>(&self, key: K) -> Option<&Value> 
        where K: AsRef<str>
    {
        for scope in self.stack.iter().rev() {
            if let Some(value) = scope.get(key.as_ref()) {
                return Some(value)
            }
        }
        None
    }

    pub fn get_super<K>(&self, key: K) -> Option<&Value>
        where K: AsRef<str>
    {
        let key = key.as_ref();
        let len = self.stack.len();
        if len > 1 {
            for scope in (&self.stack[..len - 1]).iter().rev() {
                if let Some(value) = scope.get(key) {
                    return Some(value);
                }
            }
            None
        } else {
            self.base.get(key)
        }
    }
}