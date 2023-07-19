pub use crate::node::{NodeHandle, ParseTreeNode};

fn preprocess_source(source: String) -> String {
    // add spaces around parens so they are tokenized
    source.replace("(", " ( ").replace(")", " ) ")
}

fn parse_node(token_iter: &mut std::str::SplitWhitespace, nodes: &mut NodeArena) -> (NodeHandle, bool) {
    // Returns handle to a parse tree node if one was found, and "true" if it's a list terminator.
    let token_option = token_iter.next();

    match token_option {
        None => {
            println!("EOF");
            (0, true)
        } // TODO: Crash - expecting close paren

        Some(token) => {
            // println!( "{}",  token);

            if token == "(" {
                println!("Unexpected close paren");
                return (parse_list(token_iter), false);
            } else if token == ")" {
                return (0, true);
            } else {
                // Try to parse as int; if not, treat as symbol
                match token.parse::<i32>() {
                    Ok(ival) => {
                        NodeHandleVec.
                        return (Gc::new(ParseTreeNode::Int(ival)), false);
                    }
                    Err(..) => {
                        return (
                            Gc::new(ParseTreeNode::Symbol(Gc::new(token.to_string()))),
                            false,
                        );
                    }
                }
            }
        }
    }
}

fn parse_list(token_iter: &mut std::str::SplitWhitespace) -> GcNode {
    let mut list = Vec::<Gc<ParseTreeNode>>::new();
    loop {
        let (list_node, is_terminator) = parse_node(token_iter);
        if is_terminator {
            break;
        } else {
            list.push(list_node);
        }
    }
    return Gc::new(ParseTreeNode::List(Gc::new(list)));
}

pub fn parse(source: String) -> GcNode {
    let preproc = preprocess_source(source);
    let mut tokens = preproc.split_whitespace();

    let (node, _) = parse_node(&mut tokens);
    return node;
}
