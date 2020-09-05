use std::collections::HashMap;
use std::iter::Iterator;

pub use crate::node::ParseTreeNode;
pub use crate::parse::parse_line;

pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub locals: HashMap<String, ParseTreeNode>,
}

pub fn get(scope: &Scope, key: &String) -> ParseTreeNode {
    // gets a node from the scope, or Nil if it is not found.
    match scope.locals.get(key) {
        Some(node) => {
            return node.to_owned();
        }
        None  => {
            match scope.parent {
                Some(ref parent) => {
                    return get(&parent, key);
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

pub fn set(scope: &mut Scope, key: String, value: ParseTreeNode){
    scope.locals.insert(key, value);
}
