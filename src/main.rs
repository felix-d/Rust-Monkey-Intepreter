#![feature(box_patterns)]

extern crate regex;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;
mod object;
mod evaluator;

use crate::repl::Repl;

fn main() {
    Repl::run();
}
