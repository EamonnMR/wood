use gc::{Finalize, Gc, Trace};

pub type GcNode = Gc<ParseTreeNode>;

pub type GcList = Gc<Vec<GcNode>>;

pub type GcStr = Gc<String>;
pub use crate::scope::GcScope;

#[derive(Finalize, Trace)]
pub enum ParseTreeNode {
    Symbol(GcStr),
    List(GcList),
    Int(i32),
    Nil,
    Function {
        params: GcList,
        proc: GcNode,
        closure_scope: GcScope,
    },
}

pub fn new_gclist() -> GcList {
    Gc::new(Vec::<Gc<ParseTreeNode>>::new())
}

pub fn new_nil() -> GcNode {
    Gc::new(ParseTreeNode::Nil)
}

pub fn new_blank_str() -> GcStr {
    Gc::new(String::from(""))
}

impl ParseTreeNode {
    pub fn print_node(&self, depth: usize) {
        // https://users.rust-lang.org/t/fill-string-with-repeated-character/1121/3
        let indent = std::iter::repeat(" ").take(depth).collect::<String>();

        match *self {
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
                    (*node).print_node(depth + 1);
                }
                println!("{})", indent);
            }
            ParseTreeNode::Function {
                ref params,
                ref proc,
                ref closure_scope,
            } => {
                println!("{}Lambda params (", indent);
                for node in &**params {
                    node.print_node(depth + 1);
                }
                println!("{}) proc: ", indent);
                proc.print_node(depth + 1);
                println!("{})", indent);
                println!("{}scope: ", indent);
                closure_scope.borrow().print_locals(depth + 1)
            }
            ParseTreeNode::Nil => {
                println!("{}# Nil Node", indent);
            }
        }
    }

    // "expect" functions. These are similar to Enum Methods
    // https://docs.rs/enum-methods/0.0.8/enum_methods/
    // But with the critical difference that they don't panic
    // if they don't get what they expect. I want the interpreter
    // to be able to gracefully handle the unexpected.

    pub fn expect_symbol(&self) -> GcStr {
        match &*self {
            ParseTreeNode::Symbol(string) => {
                return string.clone();
            }
            _ => {
                println!("Expected a string, got: ");
                self.print_node(20);
                return new_blank_str();
            }
        }
    }
    pub fn expect_list(&self) -> GcList {
        match &*self {
            ParseTreeNode::List(list) => {
                return list.clone();
            }
            _ => {
                println!("Expected list, got: ");
                self.print_node(20);
                return new_gclist();
            }
        }
    }

    pub fn expect_int(&self) -> i32 {
        match *self {
            ParseTreeNode::Int(int) => {
                return int;
            }
            _ => {
                println!("Expected an int, got: ");
                self.print_node(20);
                return 0;
            }
        }
    }
}

// TODO: use enum_methods?
