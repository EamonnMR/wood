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

fn get_global_state() => (Arena<Scope>, Arena<ParseTreeNode>){
    
}

fn repl() {
    // REPL
    println!("Wood 0.0.1");
    let mut scope_arena, node_arena = get_global_state();
    loop {
        let mut inputline = String::new();
        io::stdin()
            .read_line(&mut inputline)
            .expect("failed to read line");
        let root_node_handle = parse(&mut nodes, inputline);
        eval(0, scope_arena, node_arena, root_node_handle).print_node(0);
    }
}

fn run_file(file: &str) {
    let file_bytes = &fs::read(file).expect("File not found");
    let file = String::from_utf8_lossy(file_bytes).to_string();

    let mut scope_arena, node_arena = get_global_state();

    let root_node_handle = parse(node_arena;
    eval(0, scope_arena, node_arena, root_node_handle).print_node(0);
}
