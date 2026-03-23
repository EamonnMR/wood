use std::iter::Iterator;
use crate::arena::{NodeHandle, ScopeHandle, mutate_scope, mutate_scope_with_eval};
use crate::node::ParseTreeNode;
use crate::node::expect_int;
use crate::node::expect_list;
use crate::node::expect_symbol;
use crate::scope::Scope;

use crate::arena::{ARENA, add_node};

impl Scope{
    pub fn function_call(&mut self, fname: &str, argv: Vec<ParseTreeNode>) -> ParseTreeNode {
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
                    expect_int(self.eval(&expect_arg()))
                    +
                    expect_int(self.eval(&expect_arg()))
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
                            last_value = self.eval(&arg);
                        }
                    }
                }
            }

            "define" => {
                // println!("define");
                let symbol = expect_symbol(expect_arg());
                let value = self.eval(&expect_arg());
                let handle = add_node(value);
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
                let possible_func_handle:  NodeHandle = self.get(&String::from(fname));
                return ARENA.with_borrow( | arena | {
                    match *arena.deref_node(possible_func_handle).borrow(){
                        ParseTreeNode::Function { params, proc } => {
                            // Bind arguments to params in the function scope
                            // We parse the args first because we can't use self.eval after we make
                            // function scope
                            let mut args = Vec::<(ParseTreeNode, ParseTreeNode)>::new();
                            for param in params {
                                args.push((param, self.eval( &expect_arg())))
                            }
                            // Populate a new scope with args bound to params
                            let function_scope_handle: ScopeHandle = self.new_child();
                            for param_value in args {
                                let (param, value) = param_value;
                                let value_handle = add_node(value);
                                let symbol = expect_symbol(param);
                                mutate_scope(function_scope_handle, |&mut scope| {
                                    scope.set(
                                        symbol.to_owned(),
                                        value_handle,
                                    );
                                });
                            }
                            return mutate_scope_with_eval(function_scope_handle, &proc);
                        }
                        _ => {
                            println!( "expected function, got");
                            possible_func.print_node( 3 );
                            return ParseTreeNode::Symbol(String::from(""));
                        }
                    }
                });
            }
        }
    }
}

