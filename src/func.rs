use std::collections::HashMap;
use std::iter::Iterator;

use crate::eval::eval;
use crate::arena::{Arena, Handle};
use crate::node::{ParseTreeNode, NodeHandleVec};
use crate::scope::Scope;

pub fn function_call(mut arena: Arena, scopeH: Handle, fname: &str, mut argv: Vec<Handle>) -> Handle {
    let mut args_index = argv.iter();

    let mut expect_arg = || -> ParseTreeNode {
        match args_index.next() {
            Some(node) => {
                return node.clone();
            }
            None => {
                // println!("Expected an additional argument");
                return arena.nilptr();
            }
        }
    };

    let mut expect_int_arg = || -> i32 {
        arena.deref_node(eval(arena, scopeH, expect_arg())).expect_int()
    };

    match fname {
        "+" => {
            arena.add_node(ParseTreeNode::Int(expect_int_arg() + expect_int_arg()))
        }
        
        "-" => {
            arena.add_node(ParseTreeNode::Int(expect_int_arg() - expect_int_arg()))
        }

        "*" => {
            arena.add_node(ParseTreeNode::Int(expect_int_arg() * expect_int_arg()))
        }
        
        "/" => {
            arena.add_node(ParseTreeNode::Int(expect_int_arg() / expect_int_arg()))
        }

        // "car" => {
        //     expect_arg().expect_list().clone()[0].clone()
        // }
/*
        "cdr" => {
            arena.add_node(ParseTreeNode::List(expect_arg().expect_list().clone().tail()))
        }
*/
        /* cons */

        "print" => {
            (*expect_arg()).print_node(0);
            return arena.nilptr();
        }

        "begin" => {
            let mut last_handle = arena.nilptr();
            loop {
                let arg = expect_arg();
                match *arg {
                    ParseTreeNode::Nil => {
                        return last_handle;
                    }
                    _ => {
                        last_handle = eval(arena, scopeH, arg);
                    }
                }
            }
        }

        "define" => {
            // println!("define");
            let symbol = expect_arg().expect_symbol();
            let value = eval(arena, scopeH, expect_arg());
            arena.deref_scope(scopeH).set((*symbol).to_owned(), value);

            return arena.nilptr();
        }

        "locals" => {
            println!("locals");
            println!("(special builtin to debug)");
            arena.deref_scope(scopeH).print_locals(15);
            return arena.nilptr;
        }

        "quote" => {
            return expect_arg();
        }

        "lambda" => {
            return arena.add_node(ParseTreeNode::Function {
                params: expect_arg().expect_list(),
                proc: expect_arg(),
                closure_scope: scopeH,
            });
        }

        _ => {
            let possible_func = arena.deref_scope(scopeH).get(&String::from(fname));
            match &*possible_func {
                ParseTreeNode::Function {
                    params,
                    proc,
                    closure_scope,
                } => {
                    // Bind arguments to params in the function scope
                    // We parse the args first because we can't use scope.eval after we make
                    // function scope
                    let mut args = Vec::<(ParseTreeNode, Handle)>::new();
                    for param in &*params.clone() {
                        args.push((param.clone(), eval(arena, scopeH, expect_arg())));
                    }
                    // Populate a new scope with args bound to params
                    let mut function_scope = arena.deref_scope(scopeH).new_child();
                    for param_value in args {
                        let (param, value) = param_value;
                        let symbol = param.expect_symbol();
                        function_scope.set((*symbol).to_owned(), value);
                    }
                    // Evaluate the function
                    return eval(arena, arena.add_scope(function_scope), proc);
                }
                _ => {
                    println!("expected function, got");
                    possible_func.print_node(3);
                    return arena.add_node(ParseTreeNode::Symbol(""));
                }
            }
        }
    }
}
