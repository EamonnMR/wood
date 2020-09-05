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

// TODO: use enum_methods?
pub fn expect_list(node: ParseTreeNode) -> Vec<ParseTreeNode> {
    match node {
        ParseTreeNode::List(list) => {
            return list;
        }
        _ => {
            println!("Expected list, got: ");
            print_node(&node, 20);
            return Vec::<ParseTreeNode>::new()
        }
    }
}

pub fn expect_int(node: ParseTreeNode) -> i32 {
    match node {
        ParseTreeNode::Int(int) => {
            return int;
        }
        _ => {
            println!("Expected an int, got: ");
            print_node(&node, 20);
            return 0;
        }
    }
}

pub fn expect_symbol(node: ParseTreeNode) -> String {
    match node {
        ParseTreeNode::Symbol(string) => {
            return string;
        }
        _ => {
            println!("Expected a string, got: ");
            print_node(&node, 20);
            return String::from("");
        }
    }
}

