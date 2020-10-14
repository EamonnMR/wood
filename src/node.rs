use gc::{Finalize, Gc, Trace};

pub type GcNode = Gc<ParseTreeNode>;

pub type GcList = Gc<Vec<GcNode>>;

pub type GcStr = Gc<String>;


#[derive(Finalize, Trace)]
pub enum ParseTreeNode {
    Symbol(GcStr),
    List(GcList),
    Int(i32),
    Nil,
    Function{
        params: GcList,
        proc: GcNode,
    }
}

pub fn GcList_new() -> GcList {
    Gc::new( Vec::<Gc<ParseTreeNode>>::new() )
}

pub fn GetNil() -> GcNode {
    Gc::new(ParseTreeNode::Nil)
}

pub fn GetBlankStr() -> GcStr {
    Gc::new(String::from(""))
}

impl ParseTreeNode {
    pub fn print_node(&self, depth: usize) {
        // https://users.rust-lang.org/t/fill-string-with-repeated-character/1121/3
        let indent = std::iter::repeat(" ").take(depth).collect::<String>();

        match *self{
            ParseTreeNode::Symbol(ref symbol) => {
                // println!("{}Symbol: {}",indent, symbol);
                println!("{}{}", indent, symbol);
            }
            ParseTreeNode::Int(int) => {
                // println!("{}Symbol: {}",indent, symbol);
                println!("{}{}", indent, int);
            }
            ParseTreeNode::List(ref list) => {
                println!("{}(", indent);
                for node in &**list {
                    (*node).print_node( depth + 1 );
                }
                println!("{})", indent);
            }
            ParseTreeNode::Function { ref params, ref proc } => {
                println!("Lambda params (");
                for node in &**params {
                    node.print_node( depth + 1 );
                }
                println!(") proc: ");
                    proc.print_node(depth + 1);
                println!(")");
            }
            ParseTreeNode::Nil  => {
                println!("{}# Nil Node", indent);
            }
        }
    }
}

// TODO: use enum_methods?
pub fn expect_list(node: GcNode) -> GcList {
    match &*node {
        ParseTreeNode::List(list) => {
            return list.clone();
        }
        _ => {
            println!("Expected list, got: ");
            node.print_node(20);
            return Gc::new(Vec::<Gc<ParseTreeNode>>::new())
        }
    }
}

pub fn expect_int(node: GcNode) -> i32 {
    match *node {
        ParseTreeNode::Int(int) => {
            return int;
        }
        _ => {
            println!("Expected an int, got: ");
            node.print_node(20);
            return 0;
        }
    }
}

pub fn expect_symbol(node: GcNode) -> GcStr {
    match &*node {
        ParseTreeNode::Symbol(string) => {
            return string.clone();
        }
        _ => {
            println!("Expected a string, got: ");
            node.print_node(20);
            return GetBlankStr();
        }
    }
}
