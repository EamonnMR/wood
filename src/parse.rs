pub use crate::node::ParseTreeNode;

fn preprocess_source(source: String) -> String {
    // add spaces around parens so they are tokenized
    source
        .replace("(", " ( ")
        .replace(")", " ) ")
}

pub fn parse_line (source: String) -> ParseTreeNode {

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
