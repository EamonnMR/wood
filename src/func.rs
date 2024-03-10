use std::collections::HashMap;
use std::iter::Iterator;

use crate::eval::eval;
use crate::arena::Arena
use crate::node::{new_blank_str, new_nil, ParseTreeNode, NodeHandleVec};

pub fn function_call(arena: mut Arena, scope: mut Scope, fname: &str, arv) -> Handle {
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

    let mut expect_int_arg = || -> i32 {
        arena.deref_node(eval(arena, scope.own_handle, expect_arg())).expect_int()
    };

    match fname {
        "+" => {
            Arena.add_node(ParseTreeNode::Int(expect_int_arg(&scope) + expect_int_arg(&scope)))
        }
        
        "-" => {
            Arena.add_node(ParseTreeNode::Int(expect_int_arg(&scope) - expect_int_arg(&scope)))
        }

        "*" => {
            Arena.add_node(ParseTreeNode::Int(expect_int_arg(&scope) * expect_int_arg(&scope)))
        }
        
        "/" => {
            Arena.add_node(ParseTreeNode::Int(expect_int_arg(&scope) / expect_int_arg(&scope)))
        }
        /*
        "map" => {
            let args, proc, scope = expect_arg().expect_function()
            let list = expect_arg().expect_list().clone()
            return_list = new_gclist();
            for value in list:
        */

        // "car" => {
        //     expect_arg().expect_list().clone()[0].clone()
        // }
/*
        "cdr" => {
            Gc::new(ParseTreeNode::List(expect_arg().expect_list().clone().tail()))
        }
*/
        /* cons */

        "print" => {
            (*expect_arg()).print_node(0);
            return new_nil();
        }

        "begin" => {
            let mut last_handle = Arena.nilptr()
            loop {
                let arg = expect_arg();
                match *arg {
                    ParseTreeNode::Nil => {
                        return last_handle;
                    }
                    _ => {
                        last_handle = eval(scope.clone(), arg);
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
                ParseTreeNode::Function {
                    params,
                    proc,
                    closure_scope,
                } => {
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
