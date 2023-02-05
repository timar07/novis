use std::{
    collections::HashMap
};
use crate::{
    interpreter::{
        value::Value,
        runtime_error::RuntimeError
    }
};

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    inner: Vec<EnvInner>,
    level: usize
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

    /// Enter new level of environment
    pub fn enter(&mut self) -> &mut Self {
        self.inner.push(EnvInner {
            hashmap: HashMap::new(),
        });

        self.level += 1;

        return self
    }

    /// Leave current scope.
    ///
    /// # Panics
    /// Panics if already at the global scope.
    pub fn leave(&mut self) {
        if self.level == 0 {
            panic!("internal environment error (negative level)");
        }

        self.inner.pop();
        self.level -= 1;
    }

    /// Set variable value in environment.
    /// It will find the nearest variable in environment and overrides the value
    /// it contains.
    ///
    /// # Errors
    /// If variable does not exists, emits RuntimeError.
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

    /// Get variable in environment.
    /// It will automaticly find the value in all the levels.
    pub fn get(&self, name: &String) -> Option<&Value> {
        let mut env_iter = self.inner.iter().rev();
        let mut current = env_iter.next();

        while current.is_some() {
            let inner = current.expect(
                &format!("internal error at {} of environment", self.level)
            );

            if let Some(value) = inner.get(name) {
                return Some(value);
            }

            current = env_iter.next();
        }

        None
    }

    /// Define value in current level of environment.
    pub fn define(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        self.get_current_mut().define(name, value)?;
        Ok(())
    }

    fn get_current_mut(&mut self) -> &mut EnvInner {
        self.inner.get_mut(self.level).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
 struct EnvInner {
    hashmap: HashMap<String, Value>,
}

impl EnvInner {
    /// Get value of this environment level
    pub fn get(&self, name: &String) -> Option<&Value> {
        self.hashmap.get(name)
    }

    /// Define value in this environment level
    pub fn define(&mut self, name: &String, value: Value) -> Result<(), RuntimeError> {
        if self.get(name).is_none() {
            self.hashmap.insert(name.into(), value);
            Ok(())
        } else {
            Err(RuntimeError::NameRedefinition { name: name.into() })
        }
    }

    /// Get value as mutable
    pub fn get_mut(&mut self, name: &String) -> Option<&mut Value> {
        self.hashmap.get_mut(name)
    }
}
