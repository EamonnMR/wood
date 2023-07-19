//Use templates

struct Arena<T> {
  Vec<T> memory,
}

// TODO: Add GC. Right now this will just leak.

impl Arena {
  fn add(item: T) => usize {
    memory.push(item);
    return memory.size() - 1;
  }

  fn get(alloc: usize) => Option<&T>{
    return memory.get(alloc);
  }
}
