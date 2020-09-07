use std::iter::Iterator;
use crate::node::ParseTreeNode;
use crate::node::expect_int;
use crate::node::expect_list;
use crate::node::expect_symbol;
use crate::scope::Scope;

impl Scope {
    pub fn function_call(&mut self, fname: &str, argv: Vec<ParseTreeNode>) -> ParseTreeNode {
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
                    expect_int(self.eval(&expect_arg()))
                    +
                    expect_int(self.eval(&expect_arg()))
                );
            }
            "define" => {
                println!("define");
                let symbol = expect_symbol(expect_arg());
                let value = self.eval(&expect_arg());
                self.set(
                    symbol.to_owned(),
                    value,
                );

                return ParseTreeNode::Symbol( symbol.to_owned());
            }

            "locals" => {
                println!("locals");
                println!("(special builtin to debug)");
                for (key, value) in self.locals.iter(){
                    println!("{}: ", key);
                    value.print_node(20);
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
                let possible_func = self.get(&String::from(fname));
                match possible_func{
                    ParseTreeNode::Function { params, proc } => {
                        println!( "Would call function" );
                    }
                    _ => {
                        println!( "expected function, got");
                        possible_func.print_node( 3 )
                    }
                }
                return ParseTreeNode::Symbol(String::from(""));
            }
        }
    }
}

