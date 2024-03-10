//Use templates

use crate::scope::Scope
use crate::node::ParseTreeNode

struct SubArena<T> {
  Vec<T> memory,
}

type Handle = usize

// TODO: Add GC. Right now this will just leak.

impl SubArena {
  fn add(item: T) => Handle {
    memory.push(item);
    return memory.size() - 1;
  }

  fn deref(handle: Handle) => Option<&T>{
    return memory.get(handle);
  }
}

struct Arena {
  SubArena<Scope> scopes,
  SubArena<ParseTreeNode> nodes,
}

impl Arena {
  fn add_scope(scope: Scope) => Handle {
    let handle = scopes.add(scope)
    scope.own_handle = handle
    return handle
  }

  fn add_node(node: ParseTreeNode) => Handle {
    return nodes.add(node)
  }

  fn deref_scope(handle: Handle) => Opselftion(&Scope) {
    return scopes.deref(handle)
  }

  fn deref_node(handle: Handle) => Option(&ParseTreeNode) {
    return nodes.deref(handle)
  }

  fn nilptr() => Handle {
    return 0
  }

  pub fn new() => Self {
    let new_arena = Self {
      SubArena<Scope>::new(),
      SubArena<ParseTreeNode>::new()
    }
    // Create nil ptr:
    new_arena.nodes.add(ParseTreeNode::Nil::new())

    return new_arena
  }
}