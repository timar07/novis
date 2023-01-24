use std::collections::HashMap;

#[derive(Clone)]
pub struct Env {
    hashmap: HashMap<String, f64>,
    enclosing: Option<Box<Env>>
}

impl Env {
    pub fn global() -> Env {
        Env {
            hashmap: HashMap::new(),
            enclosing: None
        }
    }

    pub fn local(enclosing: &mut Env) -> Env {
        Env {
            hashmap: HashMap::new(),
            enclosing: Some(Box::new(enclosing.clone())),
        }
    }

    pub fn define(&mut self, name: String, value: f64) {
        self.hashmap.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<f64> {
        if let Some(val) = self.get_local(name) {
            return Some(*val);
        }

        let mut current_env = self.enclosing.as_ref();

        while let Some(env) = current_env {
            if let Some(value) = env.get_local(name) {
                return Some(value.clone());
            }

            current_env = current_env.unwrap().enclosing.as_ref();
        }

        None
    }

    pub fn get_local(&self, name: &String) -> Option<&f64> {
        self.hashmap.get(name)
    }
}
