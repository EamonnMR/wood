use std::collections::HashMap;

pub use crate::node::{NodeHandle, ParseTreeNode};

pub type ScopeHandle = usize;

pub struct Scope {
    pub parent: Option<ScopeHandle>,
    pub locals: HashMap<String, NodeHandle>,
    pub own_handle: ScopeHandle,
}

pub type ScoVec = Vec<Scope>

impl Scope {
    pub fn get(&self, &mut Vec<Scope> scopes, key: &String) -> NodeHandle {
        // gets a node from the scope, or Nil if it is not found.
        match self.locals.get(key) {
            Some(node) => {
                return node.clone();
            }
            None => {
                match self.parent {
                    Some(ref parent) => {
                        return scopes[parent].borrow().get(key);
                    }
                    None => {
                        // bad bad very not good
                        // we need better nil handling
                        return ParseTreeNode.nil;
                    }
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: GcNode) {
        self.locals.insert(key, value);
    }

    pub fn new() -> Scope {
        Scope {
            parent: None,
            locals: HashMap::new(),
        }
    }

    pub fn new_child(self) -> Scope {
        Scope {
            parent: Some(self.gc_of()),
            locals: HashMap::new(),
        }
    }

    pub fn gc_of(self) -> GcScope {
        Gc::new(GcCell::new(self))
    }
    pub fn print_locals(&self, indent: usize) {
        for (key, value) in self.locals.iter() {
            println!("{}: ", key);
            (*value).print_node(indent + 5);
        }
    }
}
