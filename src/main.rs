use std::io;
use std::collections::HashMap;

mod node;
mod parse;
mod scope;
mod eval;
mod func;

pub use crate::node::ParseTreeNode;
pub use crate::node::print_node;
pub use crate::parse::parse_line;
pub use crate::scope::Scope;
pub use crate::scope::get;
pub use crate::scope::set;
pub use crate::eval::eval;


fn main() {
    // REPL
    println!("Wood 0.0.1");
    let mut root_scope = Scope {
            parent: None,
            locals: HashMap::new()
    };

    loop {
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse_line ( inputline );
        print_node( &root_node, 0);
        let result = eval( &mut root_scope, &root_node );
        print_node( &result, 0);
    }
}
