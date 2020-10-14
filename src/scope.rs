use std::collections::HashMap;

pub use crate::node::{ParseTreeNode, GcNode, new_nil};

pub struct Scope <'a>{
    pub parent: Option<&'a Scope<'a>>,
    pub locals: HashMap<String, GcNode>,
}

impl Scope <'_> {
    pub fn get(&self, key: &String) -> GcNode {
        // gets a node from the scope, or Nil if it is not found.
        match self.locals.get(key) {
            Some(node) => {
                return node.clone();
            }
            None  => {
                match self.parent {
                    Some(ref parent) => {
                        return parent.get(key);
                    }
                    None => {
                        // bad bad very not good
                        // we need better nil handling
                        return new_nil();
                    }
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: GcNode) { 
        self.locals.insert(key, value);
    }

    pub fn new() -> Scope <'static> {
        Scope {
            parent: None,
            locals: HashMap::new()
        }
    }

    pub fn new_child<'a>(& 'a mut self) -> Scope<'a> {
        Scope {
            parent: Some(self),
            locals: HashMap::new()
        }
    }
}
