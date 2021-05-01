#![allow(incomplete_features)]
#![feature(if_let_guard)]
#![feature(let_chains)]

mod node;
mod token;

mod lexer;
mod parser;

use std::io::{stdin, stdout, Write};

use lexer::Lexer;
use node::Node;
use parser::{Parser, Result};

static LINES: &'static [char] = &[13 as char, 10 as char];

fn main() {
    loop {
        print!("input> ");
        stdout()
            .flush()
            .unwrap();

        let mut line = String::new();
        stdin()
            .read_line(&mut line)
            .expect("read line failed");

        let inp = line.trim_end_matches(LINES);
        if inp == "." {
            return;
        }

        let chars = inp.chars();
        let toks = Lexer::new(chars);

        print!("tokens: ");
        toks.clone()
            .for_each(
                |tok| print!("{} ", tok)
            );
        println!();

        let pars = Parser::new(toks).parse();
        match pars {
            Result::None => println!("empty"),
            Result::Ok(node) => {
                let node_view = node.clone();
                println!("nodes: {}", node_view);
                print(node_view);
                println!("eval: {}", node.eval())
            }
            Result::Err(err) => println!("error: {}", err),
        }
    }
}

fn print(ref node: Node) {
    fn print_branch(last_level: bool, last_item: bool) {
        if last_level {
            if last_item {
                print!("└")
            } else {
                print!("├")
            }
        } else {
            if last_item {
                print!(" ")
            } else {
                print!("│")
            }
        }
        print!(" ")
    }

    fn print_tree_inner(node: &Node, path: &mut Vec<bool>, last: bool) {
        path.push(last);

        let last_index = path.len() - 1;
        path
            .iter()
            .enumerate()
            .for_each(|(i, &v)| print_branch(i == last_index, v));

        match node {
            Node::Val(val) => println!("Val({})", val),
            Node::Var(name) => println!("Var({})", name),

            Node::Unary(op, node) => {
                println!("Unary({})", op);
                print_tree_inner(node, path, true)
            }
            Node::Binary(op, left, right) => {
                println!("Binary({})", op);
                print_tree_inner(left, path, false);
                print_tree_inner(right, path, true)
            }
        }

        path.pop();
    }

    let mut path: Vec<bool> = Vec::new();
    print_tree_inner(node, &mut path, true);
}
