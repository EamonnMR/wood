use crate::scope::{self, Scope};
use crate::node::ParseTreeNode;
use std::cell::RefCell;

use slotmap::{SlotMap, new_key_type};

new_key_type! {
  pub struct ScopeHandle;
}
new_key_type! {
  pub struct NodeHandle;
}

pub struct Arena {
  scopes: SlotMap<ScopeHandle, RefCell<Scope>>  ,
  nodes: SlotMap<NodeHandle, RefCell<ParseTreeNode>>,
  nullptr: NodeHandle
}

impl Arena {
  pub fn add_scope(&mut self, scope: Scope) -> ScopeHandle {
    let rc = RefCell::new(scope);
    let handle = self.scopes.insert(rc);
    mutate_scope(handle.clone(), |scope| {
      scope.handle = Some(handle);
    });
    return handle;
  }

  pub fn add_node(&mut self, node: ParseTreeNode) -> NodeHandle {
    return self.nodes.insert(RefCell::new(node));
  }

  pub fn deref_scope(&self, handle: ScopeHandle) -> & RefCell<Scope>{
    return &self.scopes[handle];
  } 

  pub fn deref_node(&self, handle: NodeHandle) -> & RefCell<ParseTreeNode> {
    return &self.nodes[handle];
  }

  pub fn nilptr(&self) -> NodeHandle {
    return self.nullptr;
  }

  pub fn new() -> Self {
    let mut nodes: SlotMap<NodeHandle, RefCell<ParseTreeNode>> = SlotMap::with_key();
    let nullptr: NodeHandle = nodes.insert(RefCell::new(ParseTreeNode::Nil));

    let new_arena: Self = { Arena {
      scopes: SlotMap::with_key(),
      nodes: nodes,
      nullptr: nullptr,
    }};
    return new_arena;
  }
}

thread_local! {
  pub static ARENA: RefCell<Arena> = RefCell::new(Arena::new());
}

pub fn mutate_scope(handle: ScopeHandle, inner: impl Fn(&mut Scope)){
  ARENA.with(|arena| {
    match arena.borrow_mut().scopes.detach(handle) {
      None => {},
      Some(scope_cell) => {
        inner(&mut scope_cell.borrow_mut());
        arena.borrow_mut().scopes.reattach(handle, scope_cell);
      }
    }
  });
}
pub fn mutate_scope_with_eval(handle: ScopeHandle, proc: &ParseTreeNode) -> ParseTreeNode{
  return ARENA.with(|arena| {
    match arena.borrow_mut().scopes.detach(handle) {
      None => {
        return ParseTreeNode::Nil;
      },
      Some(scope_cell) => {
        let node = scope_cell.borrow_mut().eval(proc);
        arena.borrow_mut().scopes.reattach(handle, scope_cell);
        return node;
      }
    }
  });
}

pub fn mutate_node(handle: NodeHandle, inner: impl Fn(&mut ParseTreeNode)){
  ARENA.with(|arena| {
    match arena.borrow_mut().nodes.detach(handle) {
      None => {},
      Some(node_cell) => {
        inner(&mut node_cell.borrow_mut());
        arena.borrow_mut().nodes.reattach(handle, node_cell);
      }
    }
  });
}

pub fn mutate_node_function<F>(handle: NodeHandle, inner: &mut F) -> ParseTreeNode
  where
    F: FnMut(&mut ParseTreeNode) -> ParseTreeNode{
  return ARENA.with_borrow_mut(|arena| {
    match arena.nodes.detach(handle) {
      None => {
        return ParseTreeNode::Nil;
      },
      Some(node_cell) => {
        let return_val: ParseTreeNode = inner(&mut node_cell.borrow_mut());
        arena.nodes.reattach(handle, node_cell);
        return return_val;
      }
    }
  });
}


pub fn add_scope(scope: Scope) -> ScopeHandle {
  return ARENA.with_borrow_mut(|arena| {
    arena.add_scope(scope)
  });
}

pub fn add_node(node: ParseTreeNode) -> NodeHandle {
  return ARENA.with_borrow_mut(|arena| {
    return arena.add_node(node)
  });
}

// pub fn deref_scope(handle: ScopeHandle) -> &Scope{
//   return ARENA.with_borrow(|arena| {
//     return arena.deref_scope(handle).borrow()
//   });
// }

// // pub fn deref_node(handle: NodeHandle) -> &mut RefCell<ParseTreeNode> {
// //   ARENA.with_borrow(|arena| {

// //   });
// // }

