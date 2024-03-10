use crate::func::function_call;
use crate::scope:Scope;
use crate::node::{new_blank_str, new_nil, ParseTreeNode, new_nil};
use crate::arena::{Arena, Handle}

pub fn eval(arena: Arena, scopeH: Handle, nodeH: Handle) -> Handle {
    match arena.deref_node(nodeH) {
        ParseTreeNode::Nil => {
            // println!("Error: nil node made it into the final parse tree");
            // Just returning something to satisfy the compiler
            // TODO: Panic! ?
            return Arena.nilptr();
        }
        ParseTreeNode::Symbol(ref symbol) => {
            // println!("Eval symbol: {}", symbol);
            return arena.deref_scope(scopeH).get(symbol);
            // TODO: Should symbols eval to themselves if they're not in scope?
            // return ParseTreeNode::Symbol(symbol.to_owned());
        }
        ParseTreeNode::Function {
            params: _,
            proc: _,
            closure_scope: _,
        } => {
            // Figure out the semantics here. I don't think we'd ever reach this...
            println!("How did this function literal get eval'd We don't have function literals!");
            return arena.nilptr();
        }
        ParseTreeNode::Int(_int) => {
            //println!("Eval int: {}", int);
            return handle;
        }
        ParseTreeNode::List(ref list) => {
            if let Some((func_name, args)) = list.split_first() {
                // TODO: Eval func_name before extracting fname - ?
                match **func_name {
                    ParseTreeNode::Symbol(ref fname) => {
                        // println!("evaluating function: {}", fname);
                        return function_call(scope, fname, (*args).to_vec());
                    }
                    _ => {
                        // TODO: Print some sort of error
                        println!("cannot parse func name - what is it?");
                        func_name.print_node(0);
                        return Gc::new(ParseTreeNode::Symbol(new_blank_str()));
                    }
                }
            } else {
                //.TODO: Some sort of error
                println!("Cannot parse fname and args from.");
                return Gc::new(ParseTreeNode::Symbol(new_blank_str()));
            }
        }
    }
}
