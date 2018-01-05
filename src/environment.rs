use std::collections::HashMap;
use values::Value;

pub struct Environment {
    stack: Vec<HashMap<String, Value>>
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            stack: vec![]
        };
        env.enter_scope();
        env
    }

    fn cur_scope(&self) -> &HashMap<String, Value> {
        let len = self.stack.len();
        &self.stack[len - 1]
    }

    fn cur_scope_mut(&mut self) -> &mut HashMap<String, Value> {
        let len = self.stack.len();
        &mut self.stack[len - 1]
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop().expect("Attempted to exit nonexistent scope.");
    }

    pub fn define<K: Into<String>>(&mut self, key: K, value: Value) {
        let scope = self.cur_scope_mut();
        scope.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        for scope in self.stack.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value)
            }
        }
        None
    }
}