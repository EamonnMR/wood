use std::io;
use std::collections::HashMap;
use std::iter::Iterator;

enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32)
}

struct Scope {
    parent: Box<Scope>,
    locals: HashMap<String, ParseTreeNode>,
}

fn call( scope: &Scope, fname: &str, argv: Vec<ParseTreeNode>) -> ParseTreeNode {
    let mut args_index = argv.iter();
    let mut pop_int = || -> i32 {
        // Apparently no matter what happens in the blocks, match returns () (?)
        let mut rval: i32 = 0;
        match args_index.next() {
            Some(node) => {
                match *node {
                    ParseTreeNode::Int(int) => { rval = int; }
                    _ => {
                        // TODO: This should throw some sort of type checking error
                    }
                }
            }
            None => {}
        }
        rval
    };

    match fname {
        "+" => { return ParseTreeNode::Int( pop_int() + pop_int() );}
        _ => { return ParseTreeNode::Symbol(String::from("")); } // TODO: Also try functions in scope
    }
}

fn call_node( node: &ParseTreeNode, scope: &Scope) -> ParseTreeNode {
	match *node{
        ParseTreeNode::Symbol(ref symbol) => {
            // TODO: Find thing in scope
            return ParseTreeNode::Symbol(symbol.to_owned());
        }    
        ParseTreeNode::Int(int) => {
            return ParseTreeNode::Int(int);
		}
        ParseTreeNode::List(ref list) => {
            if let Some((func_name, args)) = list.split_first() {
                match *func_name {
                    ParseTreeNode::Symbol( ref fname ) => {
                        let v: Vec<ParseTreeNode> = list.iter().map(
                            | x: &ParseTreeNode | -> ParseTreeNode { call_node(x, scope) }
                        ).collect();
                        return call( scope, fname, v);
                    }
                    _ => {
                        // TODO: Print some sort of error
                        return ParseTreeNode::Symbol( String::from("") );
                    } 
                }
                
            } else {
                //.TODO: Some sort of error
                return ParseTreeNode::Symbol( String::from("") );
            }
        }
    }
}
    
fn print_node( node: &ParseTreeNode, depth: usize) {
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
    }
}

fn main() {
    println!("Atmos 0.0.1");
    loop {
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse_line ( inputline );
        print_node( &root_node, 0)
    }
}

fn parse_line (source: String) -> ParseTreeNode {
    fn parse_list( token_iter: &mut std::str::SplitWhitespace ) -> ParseTreeNode {
        let mut node = ParseTreeNode::List(Vec::<ParseTreeNode>::new());
        match node {
            ParseTreeNode::List(ref mut list) => {
	        	loop {
    		        let token_option = token_iter.next();

                    match token_option {
                        None => break, // TODO: Crash - expecting close paren

                        Some( token ) => {

                            // println!( "{}",  token);

							if token == " " {
                        		continue;
                    		} else if token == "(" {
                        		list.push( parse_list( token_iter ) );
                    		} else if token == ")" {
                        		break;
                        	} else {
                                // Try to parse as int; if not, treat as symbol
                                match token.parse::<i32>(){
                                    Ok(ival) => {list.push(ParseTreeNode::Int( ival ));}
                                    Err(..) => {list.push(ParseTreeNode::Symbol( token.to_string() ));}
                                }
        		            }

						}
                    }
        		}
            }
            _ => ()

        }

        return node;
    }

    let space_added_source = source.replace("(", " ( ").replace(")", " ) ");
    // https://github.com/kballard/rfcs/blob/2d3ff42b821ab80bd6a7b3b8fda0e1c238cc7de0/active/0000-better-temporary-lifetimes.md
    let mut tokens = space_added_source.split_whitespace();

    return parse_list( &mut tokens );
}
