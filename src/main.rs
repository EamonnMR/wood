use std::io;

enum ParseTreeNode {
    Symbol(String),
    List(Vec<ParseTreeNode>),
    Int(i32)
}

fn main() {
    println!("Atmos 0.0.1");
    loop {
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline)
            .expect("failed to read line");
        let root_node = parse_line ( inputline ); 
    }
}

fn parse_line (source: String) -> ParseTreeNode {
    //fn parse_list<'token_iter, I>(vals: I) -> ParseTreeNode
    //        where I: Iterator<Item=String>
    //{
    // fn parse_list<'a>( &mut token_iter: std::str::Split<'a, &str> ) -> ParseTreeNode {
    fn parse_list( token_iter: &mut std::str::Split<&str> ) -> ParseTreeNode {
        let mut node = ParseTreeNode::List(Vec::<ParseTreeNode>::new());
        match node {
            ParseTreeNode::List(ref mut list) => {
	        	loop {
    		        let mut token_option = token_iter.next();

                    match token_option {
                        None => break, // TODO: Crash - expecting close paren

                        Some( token ) => {
							if token == " " {
                        		continue;
                    		} else if token == " ( " {
                        		list.push( parse_list( token_iter ) );
                    		} else if token == " ) " {
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
    let mut tokens = space_added_source.split(" ");

    return parse_list( &mut tokens );
}
