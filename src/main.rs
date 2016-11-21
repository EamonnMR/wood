use std::io;

enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32)
}

fn main() {
    println!("Atmos 0.0.1");
    loop {
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse_line ( inputline ); 
    }
}

fn parse_line (source: String) -> ParseTreeNode {
    let mut root_node = ParseTreeNode::List(Vec::<ParseTreeNode>::new());
    for token in source.replace("(", " ( ").replace(")", " ) ").split(" "){
        println!("{}", token );
    }
    return root_node;
}
