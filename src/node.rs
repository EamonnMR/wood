#[derive(Clone)]
pub enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32),
    Nil,
    Function{
        params: Vec<ParseTreeNode>,
        proc: Vec<ParseTreeNode>
    }
}

pub fn print_node( node: &ParseTreeNode, depth: usize) {
    // https://users.rust-lang.org/t/fill-string-with-repeated-character/1121/3
    let indent = std::iter::repeat(" ").take(depth).collect::<String>();


    match *node{
        ParseTreeNode::Symbol(ref symbol) => {
            println!("{}Symbol: {}",indent, symbol);
        }
        ParseTreeNode::Int(int) => {
            println!("{}Int: {}", indent, int);
        }
        ParseTreeNode::List(ref list) => {
            println!("{}(", indent);
            for node in list {
                print_node( node, depth + 1);
            }
            println!("{})", indent);
        }
        ParseTreeNode::Function { ref params, ref proc } => {
            println!("Lambda args (");
            for node in params {
                print_node( node, depth + 1);
            }
            println!(") proc: ");
            for node in params {
                print_node( node, depth + 1);
            }
            println!(")");
            
        }
        ParseTreeNode::Nil  => {
            println!("{}# Nil Node", indent);
        }
    }
}
