use std::{collections::HashMap};

use crate::arena::{ScopeHandle, NodeHandle, ARENA, add_scope};

pub struct Scope{
    pub parent: Option<ScopeHandle>,
    pub locals: HashMap<String, NodeHandle>,
    pub handle: Option<ScopeHandle>,
}

impl Scope{
    pub fn get(&self, key: &String) -> NodeHandle {
        // gets a node from the scope, or Nil if it is not found.
        match self.locals.get(key) {
            Some(handle) => {
                return *handle;
            }
            None  => {
                return ARENA.with_borrow(|arena| {
                    match self.parent {
                        Some(ref parent) => {
                                let parent_scope = arena.deref_scope(* parent).borrow_mut();
                                return parent_scope.get(key);
                        }
                        None => {
                            // bad bad very not good
                            // we need better nil handling
                            return arena.nilptr();
                        }
                    }
                });
            }
        }
    }

    pub fn set(&mut self, key: String, value: NodeHandle){
        self.locals.insert(key, value);
    }

    pub fn new() -> Scope {
        Scope {
            parent: None,
            locals: HashMap::new(),
            handle: None
        }
    }

    // TODO: This is all messed up.
    // Scopes need to know their handle
    pub fn new_child(&self) -> ScopeHandle {
        let new_scope = Scope {
            parent: self.handle,
            locals: HashMap::new(),
            handle: None // TODO: nullptr
        };
        return add_scope(new_scope)
    }
}
