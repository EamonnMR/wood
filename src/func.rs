use std::iter::Iterator;

use gc::{Finalize, Gc, Trace};

use crate::node::{GcList, GcNode, ParseTreeNode, GetNil, expect_int, expect_list, expect_symbol};
use crate::scope::Scope;



impl Scope <'_> {
    pub fn function_call(&mut self, fname: &str, argv: Vec<GcNode>) -> Gc<ParseTreeNode> {
        let mut args_index = argv.iter();

        let mut expect_arg = || -> GcNode {

            match args_index.next() {

                Some(node) => {
                    return *node;
                }
                None => {
                    // println!("Expected an additional argument");
                    return Gc::new(ParseTreeNode::Nil);
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
                return GetNil();
            }
            
            "begin" => {
                let mut last_value = GetNil();
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
                return last_value;
            }

            "define" => {
                // println!("define");
                let symbol = expect_symbol(expect_arg());
                let value = self.eval(expect_arg());
                self.set(
                    (*symbol).to_owned(),
                    value,
                );

                return Gc::new(ParseTreeNode::Symbol( (*symbol).to_owned()));
            }

            "locals" => {
                println!("locals");
                println!("(special builtin to debug)");
                for (key, value) in self.locals.iter(){
                    println!("{}: ", key);
                    (*value).print_node(20);
                }
                return GetNil();
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
                match *possible_func{
                    ParseTreeNode::Function { params, proc } => {
                        // Bind arguments to params in the function scope
                        // We parse the args first because we can't use self.eval after we make
                        // function scope
                        let mut args = Vec::<(GcNode, GcNode)>::new();
                        for param in &*params {
                            args.push((*param, self.eval(expect_arg())));
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
                        return function_scope.eval( proc );
                    }
                    _ => {
                        println!( "expected function, got");
                        possible_func.print_node( 3 );
                        return Gc::new(ParseTreeNode::Symbol(String::from("")));
                    }
                }
            }
        }
    }
}

