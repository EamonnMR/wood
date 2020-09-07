use std::collections::HashMap;

pub use crate::node::ParseTreeNode;
pub use crate::parse::parse_line;

pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub locals: HashMap<String, ParseTreeNode>,
}

impl Scope {
    pub fn get(&self, key: &String) -> ParseTreeNode {
        // gets a node from the scope, or Nil if it is not found.
        match self.locals.get(key) {
            Some(node) => {
                return node.to_owned();
            }
            None  => {
                match self.parent {
                    Some(ref parent) => {
                        return parent.get(key);
                    }
                    None => {
                        // bad bad very not good
                        // we need better nil handling
                        return ParseTreeNode::Nil;
                    }
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: ParseTreeNode){
        self.locals.insert(key, value);
    }

    pub fn new() -> Scope {
        Scope {
            parent: None,
            locals: HashMap::new()
        }
    }

    pub fn new_child(self) -> Scope {
        Scope {
            parent: Some(Box::new(self)),
            locals: HashMap::new()
        }
    }
}
