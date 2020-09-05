use std::io;
use std::collections::HashMap;
use std::iter::Iterator;

mod node;
mod parse;
mod scope;

pub use crate::node::ParseTreeNode;
pub use crate::node::print_node;
pub use crate::parse::parse_line;
pub use crate::scope::Scope;
pub use crate::scope::get;
pub use crate::scope::set;


fn expect_list(node: ParseTreeNode) -> Vec<ParseTreeNode> {
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

fn expect_int(node: ParseTreeNode) -> i32 {
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

fn expect_symbol(node: ParseTreeNode) -> String {
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


fn function_call( fname: &str, argv: Vec<ParseTreeNode>, scope: &mut Scope) -> ParseTreeNode {
    let mut args_index = argv.iter();

    let mut expect_arg = || -> ParseTreeNode {

        match args_index.next() {

            Some(node) => {
                return node.to_owned();
            }
            None => {
                println!("Expected an additional argument");
                return ParseTreeNode::Nil;
            }
        }
    };

    match fname {
        "+" => {
            println!("plus");

            return ParseTreeNode::Int(
                expect_int(eval(scope, &expect_arg()))
                +
                expect_int(eval(scope, &expect_arg()))
            );
        }
        "define" => {
            println!("define");
            let symbol = expect_symbol(expect_arg());
            let value = eval(scope, &expect_arg());
            set(
                scope,
                symbol.to_owned(),
                value,
            );

            return ParseTreeNode::Symbol( symbol.to_owned());
        }

        "locals" => {
            println!("locals");
            println!("(special builtin to debug)");
            for (key, value) in scope.locals.iter(){
                println!("{}: ", key);
                print_node(value, 20);
            }
            return ParseTreeNode::Nil;
        }
        
        "quote" => {
            return expect_arg();
        }
        
        "lambda" => {
            return ParseTreeNode::Function{
                // TODO: expect_list)
                params: expect_list(expect_arg()),
                proc: expect_list(expect_arg()),
            }
        }

        _ => {
            let possible_func = get(scope, &String::from(fname));
            match possible_func{
                ParseTreeNode::Function { params, proc } => {
                    println!( "Would call function" );
                }
                _ => {
                    println!( "expected function, got");
                    print_node( &possible_func, 3)
                }
            }
            return ParseTreeNode::Symbol(String::from(""));
        }
    }
}

fn eval(scope: &mut Scope,  node: &ParseTreeNode) -> ParseTreeNode {
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
                        print_node(&func_name, 0);
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
fn main() {
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

