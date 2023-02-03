use std::{
    collections::HashMap
};
use super::{value::Value, runtime_error::RuntimeError};

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub inner: Vec<EnvInner>,
    pub level: usize
}

impl Env {
    pub fn new() -> Self {
        Self {
            inner: vec![EnvInner {
                hashmap: HashMap::new(),
            }],
            level: 0
        }
    }

    pub fn enter(&mut self) -> &mut Self {
        self.inner.push(EnvInner {
            hashmap: HashMap::new(),
        });

        self.level += 1;

        return self
    }

    pub fn leave(&mut self) {
        self.inner.pop();
        self.level -= 1;
    }

    pub fn set(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        let mut level = (self.inner.len() as i32) -1;

        while level >= 0 {
            if let Some(local) = self.inner[level as usize].get_mut(name) {
                *local = value.clone();
            }

            level -= 1;
        }

        Ok(())
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        let mut env_iter = self.inner.iter().rev();
        let mut current = env_iter.next();

        while current.is_some() {
            if let Some(value) = current.unwrap().get(name) {
                return Some(value);
            }

            current = env_iter.next();
        }

        None
    }

    pub fn define(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        self.get_current_mut().define(name, value)?;
        Ok(())
    }

    fn get_current_mut(&mut self) -> &mut EnvInner {
        self.inner.get_mut(self.level).unwrap()
    }

    fn get_current(&self) -> &EnvInner {
        self.inner.get(self.level).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvInner {
    hashmap: HashMap<String, Value>,
}

impl EnvInner {
    pub fn get(&self, name: &String) -> Option<&Value> {
        self.hashmap.get(name)
    }

    pub fn define(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        self.hashmap.insert(name.into(), value);
        Ok(())
    }

    pub fn get_mut(&mut self, name: &String) -> Option<&mut Value> {
        self.hashmap.get_mut(name)
    }
}
