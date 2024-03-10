pub use crate::node::{ParseTreeNode, NodeHandleList};
pub use crate::arena::{Arena, Handle}

fn preprocess_source(source: String) -> String {
    // add spaces around parens so they are tokenized
    source.replace("(", " ( ").replace(")", " ) ")
}

fn parse_node(token_iter: &mut std::str::SplitWhitespace, arena: &mut Arena) -> (Handle, bool) {
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
                return (parse_list(token_iter), false);
            } else if token == ")" {
                return (0, true);
            } else {
                // Try to parse as int; if not, treat as symbol
                match token.parse::<i32>() {
                    Ok(ival) => {
                        NodeHandleVec.
                        return (Arena.add_node(ParseTreeNode::Int(ival)), false);
                    }
                    Err(..) => {
                        return (
                            Arena.add_node(ParseTreeNode::Symbol(token.to_string())),
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
        let (next_handle, is_terminator) = parse_node(token_iter);
        if is_terminator {
            break;
        } else {
            list.push(next_handle);
        }
    }
    return Arena.add_node(ParseTreeNode::List(list));
}

pub fn parse(arena: mut Arena, source: String) -> Handle {
    let preproc = preprocess_source(source);
    let mut tokens = preproc.split_whitespace();

    let (node, _) = parse_node(&mut tokens, &mut arena);
    return node;
}
