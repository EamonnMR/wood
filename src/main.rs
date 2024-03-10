use std::env;
use std::fs;
use std::io;

mod eval;
mod func;
mod node;
mod parse;
mod scope;
mod arena;

pub use crate::eval::eval;
pub use crate::node::ParseTreeNode;
pub use crate::parse::parse;
pub use crate::scope::Scope;
pub use crate::arena::Arena;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        run_file(&args[1])
    } else {
        repl()
    }
}

fn repl() {
    println!("Wood 0.0.2");
    let mut arena = Arena::new()
    loop {
        let mut inputline = String::new();
        io::stdin()
            .read_line(&mut inputline)
            .expect("failed to read line");
        let root_node_handle = parse(&mut arena, inputline);
        let root_scope_handle = arena.add_scope(Scope::new())
        eval(root_scope_handle, &mut arena, root_node_handle).print_node(0);
    }
}

fn run_file(file: &str) {
    let file_bytes = &fs::read(file).expect("File not found");
    let file = String::from_utf8_lossy(file_bytes).to_string();

    let mut arena = Arena::new()

    let root_node_handle = parse(&mut arena, file)
    let root_scope_handle = arena.add_scope(Scope::new())
    eval(root_scope_handle, &mut arena, root_node_handle).print_node(0);
}
