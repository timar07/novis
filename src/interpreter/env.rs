use std::{
    collections::HashMap
};
use super::{value::Value, runtime_error::RuntimeError};

#[derive(Clone, Debug, PartialEq)]
pub struct EnvInner {
    hashmap: HashMap<String, Value>,
}

impl EnvInner {
    pub fn get(&self, name: &String) -> Option<&Value> {
        self.hashmap.get(name)
    }

    pub fn set(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        if let Some(reference) = self.hashmap.get_mut(name) {
            *reference = value;
            Ok(())
        } else {
            Err(RuntimeError::NameNotDefined {
                name: name.to_string()
            })
        }
    }

    pub fn define(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        self.hashmap.insert(name.into(), value);
        Ok(())
    }

    pub fn get_mut(&mut self, name: &String) -> Option<&mut Value> {
        self.hashmap.get_mut(name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub inner: Vec<EnvInner>,
    pub level: usize
}

impl Env {
    pub fn global() -> Self {
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
        if let Some(local) = self.get_current_mut().get_mut(name) {
            *local = value;
        } else if let Some(global) = self.get_enclosing_mut() {
            global.set(name, value)?;
        }

        Ok(())
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        let mut level = (self.inner.len() as i32) -1;

        while level >= 0 {
            if let Some(local) = self.inner[level as usize].get(name) {
                return Some(local);
            }

            level -= 1;
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

    fn get_enclosing_mut(&mut self) -> Option<&mut EnvInner> {
        self.inner.iter_mut().nth(self.level-1)
    }

    fn get_enclosing(&self) -> Option<&EnvInner> {
        self.inner.iter().nth(self.level-1)
    }
}
