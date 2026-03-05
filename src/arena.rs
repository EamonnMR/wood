use crate::scope::Scope;
use crate::node::ParseTreeNode;
use std::cell::RefCell;

use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct ScopeHandle; }
new_key_type! { pub struct NodeHandle; }

pub struct Arena {
  scopes: SlotMap<ScopeHandle, RefCell<Scope>>  ,
  nodes: SlotMap<NodeHandle, RefCell<ParseTreeNode>>,
  nullptr: NodeHandle,
}

impl Arena {
  pub fn add_scope(&mut self, scope: Scope) -> ScopeHandle {
    return self.scopes.insert(RefCell::new(scope));
  }

  pub fn add_node(&mut self, node: ParseTreeNode) -> NodeHandle {
    return self.nodes.insert(RefCell::new(node));
  }

  pub fn deref_scope(&mut self, handle: ScopeHandle) -> &mut RefCell<Scope>{
    return &mut self.scopes[handle];
  } 

  pub fn deref_node(&mut self, handle: NodeHandle) -> &mut RefCell<ParseTreeNode> {
    return &mut self.nodes[handle];
  }

  pub fn nilptr(self) -> NodeHandle {
    return self.nullptr;
  }

  pub fn new() -> Self {
    let mut nodes: SlotMap<NodeHandle, RefCell<ParseTreeNode>> = SlotMap::with_key();
    let nullptr: NodeHandle = nodes.insert(RefCell::new(ParseTreeNode::Nil));

    let new_arena: Self = { Arena {
      scopes: SlotMap::with_key(),
      nodes: nodes,
      nullptr: nullptr
    }};

    return new_arena;
  }
}