use std::io;

mod node;
mod parse;
mod scope;
mod eval;
mod func;

pub use crate::node::ParseTreeNode;
// pub use crate::node::print_node;
pub use crate::parse::parse_line;
pub use crate::scope::Scope;


fn main() {
    // REPL
    println!("Wood 0.0.1");
    let mut root_scope = Scope::new();
    loop {
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse_line ( inputline );
        root_node.print_node(0);
        let result = root_scope.eval(&root_node );
        result.print_node(0);

    }
}
