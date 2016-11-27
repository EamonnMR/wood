use std::io;

enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32)
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
		                        list.push( ParseTreeNode::Symbol( token.to_string() ));
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
