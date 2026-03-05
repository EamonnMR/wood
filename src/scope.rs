use std::collections::HashMap;

use crate::arena::{ScopeHandle, NodeHandle, Arena};

pub struct Scope{
    pub parent: Option<ScopeHandle>,
    pub locals: HashMap<String, NodeHandle>,
}

impl Scope{
    pub fn get(&self, arena: &mut Arena, key: &String) -> NodeHandle {
        // gets a node from the scope, or Nil if it is not found.
        match self.locals.get(key) {
            Some(handle) => {
                return *handle;
            }
            None  => {
                match self.parent {
                    Some(ref parent) => {
                        let parent_scope = arena.deref_scope(* parent).borrow_mut();
                        return parent_scope.get(arena, key);
                    }
                    None => {
                        // bad bad very not good
                        // we need better nil handling
                        return arena.nilptr();
                    }
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: NodeHandle){
        self.locals.insert(key, value);
    }

    pub fn new() -> Scope {
        Scope {
            parent: None,
            locals: HashMap::new()
        }
    }

    pub fn new_child<'a>(& 'a mut self) -> Scope {
        Scope {
            parent: Some(self),
            locals: HashMap::new()
        }
    }
}
