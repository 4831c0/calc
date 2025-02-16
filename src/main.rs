mod token;
mod tree;
mod engine;

use std::fs::File;
use crate::engine::tree_to_instructions;
use crate::token::tokenize;
use crate::tree::tokens_to_tree;

fn main() {
    let file = File::open("input.txt").unwrap();
    let tokens = tokenize(file).unwrap();
    println!("=== [tokens] ===\n{:#?}", tokens);
    let tree = tokens_to_tree(tokens).unwrap();
    println!("=== [parse tree] ===\n{:#?}", tree);
    let insns = tree_to_instructions(tree).unwrap();
    println!("=== [instructions] ===\n{:#?}", insns);
}
