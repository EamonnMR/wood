use crate::scope::Scope;
use crate::node::ParseTreeNode;

impl Scope <'_>{
    pub fn eval(&mut self, node: &ParseTreeNode) -> ParseTreeNode {
        match *node{
            ParseTreeNode::Nil=> {
                // println!("Error: nil node made it into the final parse tree");
                // Just returning something to satisfy the compiler
                // TODO: Panic! ?
                return ParseTreeNode::Nil;
            }
            ParseTreeNode::Symbol(ref symbol) => {
                // println!("Eval symbol: {}", symbol);
                return self.get(symbol);
                // TODO: Should symbols eval to themselves if they're not in scope?
                // return ParseTreeNode::Symbol(symbol.to_owned());
            }
            ParseTreeNode::Function { params: _, proc: _ } => {
                // Figure out the semantics here. I don't think we'd ever reach this...
                //println!("How did this function literal get eval'd We don't have function literals!");
                return ParseTreeNode::Nil;
            }
            ParseTreeNode::Int(int) => {
                //println!("Eval int: {}", int);
                return ParseTreeNode::Int(int);
            }
            ParseTreeNode::List(ref list) => {
                if let Some((func_name, args)) = list.split_first() {
                    // TODO: Eval func_name before extracting fname - ?
                    match *func_name {
                        ParseTreeNode::Symbol( ref fname ) => {
                            // println!("evaluating function: {}", fname);
                            return self.function_call(fname, args.to_vec());
                        }
                        _ => {
                            // TODO: Print some sort of error
                            println!("cannot parse func name - what is it?");
                            func_name.print_node(0);
                            return ParseTreeNode::Symbol( String::from("") );
                        } 
                    }
                    
                } else {
                    //.TODO: Some sort of error
                    println!("Cannot parse fname and args from.");
                    return ParseTreeNode::Symbol( String::from("") );
                }
            }
        }
    }
}