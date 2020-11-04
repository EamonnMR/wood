use std::iter::Iterator;
use std::collections::HashMap;

use gc::{Gc, GcCell};

use crate::node::{new_blank_str, new_nil, GcNode, ParseTreeNode};
use crate::scope::{Scope, GcScope};
use crate::eval::eval;

pub fn function_call(scope: GcScope, fname: &str, argv: Vec<GcNode>) -> GcNode {
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
                eval(scope.clone(), expect_arg()).expect_int() + eval(scope.clone(), expect_arg()).expect_int(),
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
                        last_value = eval(scope.clone(), arg);
                    }
                }
            }
        }

        "define" => {
            // println!("define");
            let symbol = expect_arg().expect_symbol();
            let value = eval(scope.clone(), expect_arg());
            scope.borrow_mut().set((*symbol).to_owned(), value);

            return new_nil();
        }

        "locals" => {
            println!("locals");
            println!("(special builtin to debug)");
            scope.borrow().print_locals(15);
            return new_nil();
        }

        "quote" => {
            return expect_arg();
        }

        "lambda" => {
            return Gc::new(ParseTreeNode::Function {
                params: expect_arg().expect_list(),
                proc: expect_arg(),
                closure_scope: scope.clone(),
            });
        }

        _ => {
            let possible_func = scope.borrow().get(&String::from(fname));
            match &*possible_func {
                ParseTreeNode::Function { params, proc, closure_scope } => {
                    // Bind arguments to params in the function scope
                    // We parse the args first because we can't use scope.eval after we make
                    // function scope
                    let mut args = Vec::<(GcNode, GcNode)>::new();
                    for param in &*params.clone() {
                        args.push((param.clone(), eval(scope.clone(), expect_arg())));
                    }
                    // Populate a new scope with args bound to params
                    let mut function_scope = Scope {
                        parent: Some(closure_scope.clone()),
                        locals: HashMap::new(),
                    };
                    for param_value in args {
                        let (param, value) = param_value;
                        let symbol = param.expect_symbol();
                        function_scope.set((*symbol).to_owned(), value);
                    }
                    // Evaluate the function
                    return eval(Gc::new(GcCell::new(function_scope)), proc.clone());
                }
                _ => {
                    println!("expected function, got");
                    possible_func.print_node(3);
                    return Gc::new(ParseTreeNode::Symbol(new_blank_str()));
                }
            }
        }
    }
}
