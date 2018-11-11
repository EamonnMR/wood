use std::io;
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Clone)]
enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32),
    Nil(bool),
}

struct Scope {
    parent: Option<Box<Scope>>,
    locals: HashMap<String, ParseTreeNode>,
}

fn get(scope: Scope, key: &String) -> ParseTreeNode {
    // gets a node from the scope, or Nil if it is not found.
    match scope.locals.get(key) {
        Some(node) => {
            return node.to_owned();
        }
        None  => {
            match scope.parent {
                Some(parent) => {
                    return get(*parent, key);
                }
                None => {
                    // bad bad very not good
                    // we need better nil handling
                    return ParseTreeNode::Nil(false);
                }
            }
        }
    }
}

fn set(mut scope: Scope, key: String, value: ParseTreeNode){
    scope.locals.insert(key, value);
}

fn expect_int(node: ParseTreeNode) -> i32 {
    let mut rval: i32;
    match node {
        ParseTreeNode::Int(int) => {
            return int;
        }
        _ => {
            println!("Expected an int, got: ");
            return 0;
        }
    }
}


fn function_call( fname: &str, argv: Vec<ParseTreeNode>, mut scope: Scope) -> ParseTreeNode {
    let mut args_index = argv.iter();

    let mut expect_arg = || -> ParseTreeNode {

        match args_index.next() {

            Some(node) => {
                return *node;
            }
            None => {
                println!("Expected an additional argument");
                return ParseTreeNode::Nil(false);
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
        _ => {
            println!("unknown func {}", fname);
            return ParseTreeNode::Symbol(String::from(""));
        } // TODO: Also try functions in scope
    }
}

fn eval(mut scope: Scope,  node: &ParseTreeNode) -> ParseTreeNode {
	match *node{
        ParseTreeNode::Nil(ref nothing) => {
            println!("Error: nil node made it into the final parse tree");
            // Just returning something to satisfy the compiler
            return ParseTreeNode::Nil(*nothing);
        }
        ParseTreeNode::Symbol(ref symbol) => {
            println!("Eval symbol: {}", symbol);
            return get(scope, symbol);
            // TODO: Should symbols eval to themselves if they're not in scope?
            // return ParseTreeNode::Symbol(symbol.to_owned());
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
        ParseTreeNode::Nil(ref nothing) => {
            println!("{}# Nil Node", indent);
        }
    }
}

fn main() {
    println!("Atmos 0.0.1");
    // let mut root_scope = Scope {parent: None, locals: HashMap::new()};
    loop {
        let mut root_scope = Scope {
            parent: None,
            locals: HashMap::new()};
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse_line ( inputline );
        print_node( &root_node, 0);
        let result = eval( root_scope, &root_node );
        print_node( &result, 0);}}

fn parse_line (source: String) -> ParseTreeNode {

    fn parse_node( token_iter: &mut std::str::SplitWhitespace ) -> (ParseTreeNode, bool) {
        // Returns a parse tree node if one was found, and "true" if it's a list terminator.
        let token_option = token_iter.next();

        match token_option {
            None => {
                println!("EOF");
                (ParseTreeNode::Nil(false), true)
            } // TODO: Crash - expecting close paren

            Some( token ) => {
                // println!( "{}",  token);

                if token == "(" {
                    return (parse_list( token_iter ), false);
                } else if token == ")" {
                    return (ParseTreeNode::Nil(false), true);
                } else {
                    // Try to parse as int; if not, treat as symbol
                    match token.parse::<i32>(){
                        Ok(ival) => {
                            return (ParseTreeNode::Int( ival ), false);
                        }
                        Err(..) => {
                            return (ParseTreeNode::Symbol( token.to_string() ), false);
                        }
                    }
                }
            }
        }
    }

    fn parse_list( token_iter: &mut std::str::SplitWhitespace ) -> ParseTreeNode {
        let mut node = ParseTreeNode::List(Vec::<ParseTreeNode>::new());
        match node {
            ParseTreeNode::List(ref mut list) => {
	        	loop {
                    let (list_node, is_terminator) = parse_node(token_iter);
                    if is_terminator {
                        break;
                    } else {
                        list.push(list_node);
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

    let (node, _) = parse_node( &mut tokens );
    return node;
}

