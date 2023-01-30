use std::{
    collections::HashMap,
    borrow::{Borrow}
};
use super::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    hashmap: HashMap<String, Value>,
    pub enclosing: Option<Box<Env>>
}

impl Env {
    pub fn global() -> Env {
        Env {
            hashmap: HashMap::new(),
            enclosing: None
        }
    }

    pub fn local(enclosing: Box<Env>) -> Env {
        Env {
            hashmap: HashMap::new(),
            enclosing: Some(enclosing)
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
        self.hashmap.get_mut(name)
    }

    pub fn get_enclosing(&mut self) -> Env {
        self.enclosing.as_ref().unwrap().as_ref().clone()
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.hashmap.insert(name, value);
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
        self.hashmap.get(name)
    }
}
