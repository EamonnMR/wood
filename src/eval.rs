use crate::scope::Scope;
use crate::scope::get;
use crate::scope::set;
use crate::node::ParseTreeNode;
// use crate::node::print_node;
use crate::func::function_call;

pub fn eval(scope: &mut Scope,  node: &ParseTreeNode) -> ParseTreeNode {
	match *node{
        ParseTreeNode::Nil=> {
            println!("Error: nil node made it into the final parse tree");
            // Just returning something to satisfy the compiler
            return ParseTreeNode::Nil;
        }
        ParseTreeNode::Symbol(ref symbol) => {
            println!("Eval symbol: {}", symbol);
            return get(scope, symbol);
            // TODO: Should symbols eval to themselves if they're not in scope?
            // return ParseTreeNode::Symbol(symbol.to_owned());
        }
        ParseTreeNode::Function { ref params, ref proc } => {
            return ParseTreeNode::Nil;
        }
        ParseTreeNode::Int(int) => {
            println!("Eval int: {}", int);
            return ParseTreeNode::Int(int);
		}
        ParseTreeNode::List(ref list) => {
            if let Some((func_name, args)) = list.split_first() {
                // TODO: Eval func_name before extracting fname - ?
                match *func_name {
                    ParseTreeNode::Symbol( ref fname ) => {
                        println!("evaluating function: {}", fname);
                        // TODO: Custom functions may need something like this:
                        //let v: Vec<ParseTreeNode> = args.iter().map(
                        //    | x: &ParseTreeNode | -> ParseTreeNode { eval(scope, x) }
                        //).collect();
                        return function_call(fname, args.to_vec(), scope);
                    }
                    _ => {
                        // TODO: Print some sort of error
                        println!("cannot parse func name");
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
