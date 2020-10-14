pub use crate::node::{ParseTreeNode, GcNode, GcList, new_gclist};
use gc::{Gc};

fn preprocess_source(source: String) -> String {
    // add spaces around parens so they are tokenized
    source
        .replace("(", " ( ")
        .replace(")", " ) ")
}

fn parse_node( token_iter: &mut std::str::SplitWhitespace ) -> (GcNode, bool) {
    // Returns a GC'd pointer to a parse tree node if one was found, and "true" if it's a list terminator.
    let token_option = token_iter.next();

    match token_option {
        None => {
            println!("EOF");
            (Gc::new(ParseTreeNode::Nil), true)
        } // TODO: Crash - expecting close paren

        Some( token ) => {
            // println!( "{}",  token);

            if token == "(" {
                return (parse_list( token_iter ), false);
            } else if token == ")" {
                return (Gc::new(ParseTreeNode::Nil), true);
            } else {
                // Try to parse as int; if not, treat as symbol
                match token.parse::<i32>(){
                    Ok(ival) => {
                        return (Gc::new(ParseTreeNode::Int( ival )), false);
                    }
                    Err(..) => {
                        return (Gc::new(ParseTreeNode::Symbol( Gc::new(token.to_string()) )), false);
                    }
                }
            }
        }
    }
}

fn parse_list( token_iter: &mut std::str::SplitWhitespace ) -> GcNode {
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

pub fn parse (source: String) -> GcNode {
    let preproc = preprocess_source(source);
    let mut tokens = preproc.split_whitespace();

    let (node, _) = parse_node( &mut tokens );
    return node;
}

