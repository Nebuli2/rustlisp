use std::collections::HashMap;
use super::Value;

type Scope = HashMap<String, Value>;
type StructFields = Vec<String>;

pub trait FieldIndex {
    fn index<K: AsRef<str>>(&self, K) -> Option<usize>;
}

impl FieldIndex for StructFields {
    fn index<K: AsRef<str>>(&self, key: K) -> Option<usize> {
        let key = key.as_ref();
        for (i, k) in self.iter().enumerate() {
            if k == key {
                return Some(i);
            }
        }
        None
    }
}

pub struct Environment {
    base: Scope,
    stack: Vec<Scope>,
    structs: HashMap<String, StructFields>,
}

impl Default for Environment {
    fn default() -> Self {
        let mut env = Environment {
            base: HashMap::new(),
            stack: vec![],
            structs: HashMap::new(),
        };
        env.enter_scope();
        env
    }
}

impl Environment {
    pub fn structs(&self) -> &HashMap<String, StructFields> {
        &self.structs
    }

    pub fn structs_mut(&mut self) -> &mut HashMap<String, StructFields> {
        &mut self.structs
    }

    pub fn add_struct<S: Into<String>>(&mut self, name: S, fields: StructFields) {
        self.structs_mut().insert(name.into(), fields);
    }

    pub fn get_struct<S: Into<String>>(&self, name: S) -> Option<&StructFields> {
        let name = name.into();
        let fields = self.structs().get(&name);
        match fields {
            Some(fields) => Some(fields),
            _ => None,
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
        self.stack
            .pop()
            .expect("Attempted to exit nonexistent scope.");
    }

    pub fn define<K>(&mut self, key: K, value: Value)
    where
        K: Into<String>,
    {
        let scope = self.cur_scope_mut();
        scope.insert(key.into(), value);
    }

    pub fn get<K>(&self, key: K) -> Option<&Value>
    where
        K: AsRef<str>,
    {
        for scope in self.stack.iter().rev() {
            if let Some(value) = scope.get(key.as_ref()) {
                return Some(value);
            }
        }
        None
    }

    pub fn get_super<K>(&self, key: K) -> Option<&Value>
    where
        K: AsRef<str>,
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
