use crate::scope::Scope;
use crate::node::ParseTreeNode;

use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct ScopeHandle; }
new_key_type! { pub struct NodeHandle; }

pub struct Arena {
  scopes: SlotMap<ScopeHandle, Scope>,
  nodes: SlotMap<NodeHandle, ParseTreeNode>,
  nullptr: NodeHandle,
}

impl Arena {
  pub fn add_scope(&mut self, scope: Scope) -> ScopeHandle {
    return self.scopes.insert(scope);
  }

  pub fn add_node(&mut self, node: ParseTreeNode) -> NodeHandle {
    return self.nodes.insert(node)
  }

  pub fn deref_scope(&mut self, handle: ScopeHandle) -> &mut Scope{
    return &mut self.scopes[handle];
  }

  pub fn deref_node(&mut self, handle: NodeHandle) -> &mut ParseTreeNode {
    return &mut self.nodes[handle];
  }

  pub fn nilptr(self) -> NodeHandle {
    return self.nullptr;
  }

  pub fn new() -> Self {
    let mut nodes: SlotMap<NodeHandle, ParseTreeNode> = SlotMap::with_key();
    let nullptr: NodeHandle = nodes.insert(ParseTreeNode::Nil);

    let new_arena: Self = { Arena {
      scopes: SlotMap::with_key(),
      nodes: nodes,
      nullptr: nullptr
    }};

    return new_arena;
  }
}