use std::io;
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Clone)]
enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32),
    Nil,
    Function{
        params: Vec<ParseTreeNode>,
        proc: Vec<ParseTreeNode>
    }
}

struct Scope {
    parent: Option<Box<Scope>>,
    locals: HashMap<String, ParseTreeNode>,
}

fn preprocess_source(source: String) -> String {
    // add spaces around parens so they are tokenized
    source
        .replace("(", " ( ")
        .replace(")", " ) ")
}

fn get(scope: &Scope, key: &String) -> ParseTreeNode {
    // gets a node from the scope, or Nil if it is not found.
    match scope.locals.get(key) {
        Some(node) => {
            return node.to_owned();
        }
        None  => {
            match scope.parent {
                Some(ref parent) => {
                    return get(&parent, key);
                }
                None => {
                    // bad bad very not good
                    // we need better nil handling
                    return ParseTreeNode::Nil;
                }
            }
        }
    }
}

fn set(scope: &mut Scope, key: String, value: ParseTreeNode){
    scope.locals.insert(key, value);
}

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
fn main() {
    println!("Wood 0.0.1");
    // let mut root_scope = Scope {parent: None, locals: HashMap::new()};
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
        print_node( &result, 0);}}

fn parse_line (source: String) -> ParseTreeNode {

    fn parse_node( token_iter: &mut std::str::SplitWhitespace ) -> (ParseTreeNode, bool) {
        // Returns a parse tree node if one was found, and "true" if it's a list terminator.
        let token_option = token_iter.next();

        match token_option {
            None => {
                println!("EOF");
                (ParseTreeNode::Nil, true)
            } // TODO: Crash - expecting close paren

            Some( token ) => {
                // println!( "{}",  token);

                if token == "(" {
                    return (parse_list( token_iter ), false);
                } else if token == ")" {
                    return (ParseTreeNode::Nil, true);
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

    let preproc = preprocess_source(source);
    let mut tokens = preproc.split_whitespace();

    let (node, _) = parse_node( &mut tokens );
    return node;
}

