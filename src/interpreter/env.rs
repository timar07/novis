use std::{
    collections::HashMap,
    borrow::{Borrow}
};
use super::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct EnvInner {
    hashmap: HashMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    pub inner: Box<EnvInner>,
    pub enclosing: Option<Box<Env>>
}

impl Env {
    pub fn global() -> Env {
        Env {
            inner: Box::new(EnvInner {
                hashmap: HashMap::new(),
            }),
            enclosing: None
        }
    }

    pub fn local(enclosing: Box<Env>) -> Env {
        Env {
            inner: Box::new(EnvInner {
                hashmap: HashMap::new(),
            }),
            enclosing: Some(enclosing)
        }
    }

    pub fn leave(&mut self) {
        if let Some(global) = self.enclosing.as_deref_mut() {
            *global = Env {
                inner: self.inner.clone(),
                enclosing: None
            };
        }
    }

    pub fn set(&mut self, name: &String, value: Value) {
        if let Some(local) = self.get_local_mut(name) {
            *local = value;
        } else if let Some(global) = self.enclosing.as_mut() {
            global.set(name, value);
        }
    }

    fn get_local_mut(&mut self, name: &String) -> Option<&mut Value> {
        self.inner.as_mut().hashmap.get_mut(name)
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.inner.as_mut().hashmap.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        if let Some(local) = self.get_local(name) {
            return Some(local);
        }

        if let Some(global) = &self.enclosing {
            return global.as_ref().borrow().get(name);
        }

        None
    }

    pub fn get_local(&self, name: &String) -> Option<&Value> {
        self.inner.as_ref().hashmap.get(name)
    }
}
