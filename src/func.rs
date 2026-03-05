use std::iter::Iterator;
use crate::node::ParseTreeNode;
use crate::node::expect_int;
use crate::node::expect_list;
use crate::node::expect_symbol;
use crate::scope::Scope;
use std::cell::RefCell;

use crate::arena::{Arena};
impl Scope{
    pub fn function_call(&mut self, fname: &str, arena: &mut Arena, argv: Vec<ParseTreeNode>) -> ParseTreeNode {
        let mut args_index = argv.iter();

        let mut expect_arg = || -> ParseTreeNode {

            match args_index.next() {

                Some(node) => {
                    return node.to_owned();
                }
                None => {
                    // println!("Expected an additional argument");
                    return ParseTreeNode::Nil;
                }
            }
        };

        match fname {
            "+" => {
                // println!("plus");

                return ParseTreeNode::Int(
                    expect_int(self.eval(arena, &expect_arg()))
                    +
                    expect_int(self.eval(arena, &expect_arg()))
                );
            }
            
            "print" => {
                expect_arg().print_node(0);
                return ParseTreeNode::Nil;
            }
            
            "begin" => {
                let mut last_value = ParseTreeNode::Nil;
                loop {
                    let arg = expect_arg();
                    match arg {
                        ParseTreeNode::Nil => {
                            return last_value;
                        }
                        _ => {
                            last_value = self.eval(arena, &arg);
                        }
                    }
                }
            }

            "define" => {
                // println!("define");
                let symbol = expect_symbol(expect_arg());
                let value = self.eval(arena, &expect_arg());
                let handle = arena.add_node(value);
                self.set(
                    symbol.to_owned(),
                    handle,
                );

                return ParseTreeNode::Symbol( symbol.to_owned());
            }

            "locals" => {
                println!("locals");
                println!("(special builtin to debug)");
                for (key, value) in self.locals.iter(){
                    println!("{}: ", key);
                    // value.print_node(20);
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
                    proc: Box::new(expect_arg()),
                }
            }

            _ => {
                let possible_func = self.get(arena, &String::from(fname));
                match possible_func{
                    ParseTreeNode::Function { params, proc } => {
                        // Bind arguments to params in the function scope
                        // We parse the args first because we can't use self.eval after we make
                        // function scope
                        let mut args = Vec::<(ParseTreeNode, ParseTreeNode)>::new();
                        for param in params {
                            args.push((param, self.eval(arena, &expect_arg())))
                        }
                        // Populate a new scope with args bound to params
                        let mut function_scope = self.new_child();
                        for param_value in args {
                            let (param, value) = param_value;
                            let symbol = expect_symbol(param);
                            function_scope.set(
                                symbol.to_owned(),
                                value,
                            );
                        }
                        // Evaluate the function
                        return function_scope.eval( arena,&proc );
                    }
                    _ => {
                        println!( "expected function, got");
                        possible_func.print_node( 3 );
                        return ParseTreeNode::Symbol(String::from(""));
                    }
                }
            }
        }
    }
}

