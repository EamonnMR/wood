use std::collections::HashMap;

pub use crate::node::ParseTreeNode;
pub use crate::arena::{Arena, Handle};


pub struct Scope {
    pub parent: Option<Handle>,
    pub locals: HashMap<String, Handle>,
    pub own_handle: Handle,
}

pub type ScoVec = Vec<Scope>;

impl Scope {
    pub fn get(&self, arena: &Arena, key: &String) -> Handle {
        // gets a node from the scope, or Nil if it is not found.
        match self.locals.get(key) {
            Some(node) => {
                return node.clone();
            }
            None => {
                match self.parent {
                    Some(ref parent) => {
                        match arena.deref_scope(parent) {
                            Some(parent_scope) => parent_scope.get(key),
                            None => return arena.nilptr()
                        }
                    }
                    None => {
                        // bad bad very not good
                        // we need better nil handling
                        return arena.nilptr()
                    }
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: ParseTreeNode) {
        self.locals.insert(key, value);
    }

    pub fn new() -> Scope {
        Scope {
            parent: None,
            locals: HashMap::new(),
            own_handle: None
        }
    }

    pub fn new_child(self) -> Scope {
        return Scope {
            parent: Some(self.own_handle),
            locals: HashMap::new(),
            own_handle: None
        };
    }

    pub fn print_locals(&self, indent: usize) {
        for (key, value) in self.locals.iter() {
            println!("{}: ", key);
            (*value).print_node(indent + 5);
        }
    }
}
