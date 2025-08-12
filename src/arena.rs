//Use templates

use crate::scope::Scope;
use crate::node::ParseTreeNode;

struct SubArena<T> {
  memory: Vec<T>,
}

pub type Handle = usize;

// TODO: Add garbage collection. Right now this will just leak.

impl<T> SubArena<T> {
  fn add(self, item: T) -> Handle {
    self.memory.push(item);
    return self.memory.size() - 1;
  }

  fn deref(self, handle: Handle) -> Option<&'static T>{
    return self.memory.get(handle);
  }
}

pub struct Arena {
  scopes: SubArena<Scope>,
  nodes: SubArena<ParseTreeNode>,
}

impl Arena {
  pub fn add_scope(self, mut scope: Scope) -> Handle {
    let handle = self.scopes.add(scope);
    scope.own_handle = handle;
    return handle;
  }

  pub fn add_node(self, node: ParseTreeNode) -> Handle {
    return self.nodes.add(node)
  }

  pub fn deref_scope(self, handle: Handle) -> Option<&'static Scope> {
    return self.scopes.deref(handle);
  }

  pub fn deref_node(self, handle: Handle) -> Option<&'static ParseTreeNode> {
    return self.nodes.deref(handle);
  }

  pub fn nilptr(self) -> Handle {
    return 0;
  }

  pub fn new() -> Self {
    let new_arena = Self {
      scopes: SubArena::new(),
      nodes: SubArena::new(),
    };
    // Create nil ptr:
    new_arena.add_node(ParseTreeNode::Nil);

    return new_arena;
  }
}