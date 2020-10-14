use std::iter::Iterator;

use gc::{Gc};

use crate::node::{GcNode, ParseTreeNode, new_nil, expect_int, expect_list, expect_symbol, new_blank_str};
use crate::scope::Scope;



impl Scope <'_> {
    pub fn function_call(&mut self, fname: &str, argv: Vec<GcNode>) -> GcNode {
        let mut args_index = argv.iter();

        let mut expect_arg = || -> GcNode {

            match args_index.next() {

                Some(node) => {
                    return node.clone();
                }
                None => {
                    // println!("Expected an additional argument");
                    return new_nil();
                }
            }
        };

        match fname {
            "+" => {
                // println!("plus");

                return Gc::new(ParseTreeNode::Int(
                    expect_int(self.eval(expect_arg()))
                    +
                    expect_int(self.eval(expect_arg()))
                ));
            }
            
            "print" => {
                (*expect_arg()).print_node(0);
                return new_nil();
            }
            
            "begin" => {
                let mut last_value = new_nil();
                loop {
                    let arg = expect_arg();
                    match *arg {
                        ParseTreeNode::Nil => {
                            return last_value;
                        }
                        _ => {
                            last_value = self.eval(arg);
                        }
                    }
                }
            }

            "define" => {
                // println!("define");
                let symbol = expect_symbol(expect_arg());
                let value = self.eval(expect_arg());
                self.set(
                    (*symbol).to_owned(),
                    value,
                );

                return new_nil()
            }

            "locals" => {
                println!("locals");
                println!("(special builtin to debug)");
                for (key, value) in self.locals.iter(){
                    println!("{}: ", key);
                    (*value).print_node(20);
                }
                return new_nil();
            }
            
            "quote" => {
                return expect_arg();
            }
            
            "lambda" => {
                return Gc::new(ParseTreeNode::Function{
                    // TODO: expect_list)
                    params: expect_list(expect_arg()),
                    proc: expect_arg(),
                })
            }

            _ => {
                let possible_func = self.get(&String::from(fname));
                match &*possible_func{
                    ParseTreeNode::Function { params, proc } => {
                        // Bind arguments to params in the function scope
                        // We parse the args first because we can't use self.eval after we make
                        // function scope
                        let mut args = Vec::<(GcNode, GcNode)>::new();
                        for param in &*params.clone() {
                            args.push((param.clone(), self.eval(expect_arg())));
                        }
                        // Populate a new scope with args bound to params
                        let mut function_scope = self.new_child();
                        for param_value in args {
                            let (param, value) = param_value;
                            let symbol = expect_symbol(param);
                            function_scope.set(
                                (*symbol).to_owned(),
                                value,
                            );
                        }
                        // Evaluate the function
                        return function_scope.eval( proc.clone() );
                    }
                    _ => {
                        println!( "expected function, got");
                        possible_func.print_node( 3 );
                        return Gc::new(ParseTreeNode::Symbol(new_blank_str()));
                    }
                }
            }
        }
    }
}

