use std::env;
use std::fs;
use std::io;

mod eval;
mod func;
mod node;
mod parse;
mod scope;

pub use crate::node::ParseTreeNode;
pub use crate::parse::parse;
pub use crate::scope::Scope;
pub use crate::eval::eval;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        run_file(&args[1])
    } else {
        repl()
    }
}

fn repl() {
    // REPL
    println!("Wood 0.0.1");
    let root_scope = Scope::new().gc_of();
    loop {
        let mut inputline = String::new();
        io::stdin()
            .read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse(inputline);
        eval(root_scope.clone(), root_node).print_node(0);
    }
}

fn run_file(file: &str) {
    let file_bytes = &fs::read(file).expect("File not found");
    let file = String::from_utf8_lossy(file_bytes).to_string();
    let root_node = parse(file);
    let root_scope = Scope::new().gc_of();
    eval(root_scope.clone(), root_node).print_node(0);
}
