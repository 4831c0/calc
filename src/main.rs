mod bytecode;
mod token;
mod tree;
mod vm;

use crate::bytecode::tree_to_instructions;
use crate::token::tokenize;
use crate::tree::tokens_to_tree;
use crate::vm::run;
use std::fs::File;

fn main() {
    let file = File::open("input.txt").unwrap();
    let tokens = tokenize(file).unwrap();
    println!("=== [tokens] ===\n{:#?}", tokens);
    let tree = tokens_to_tree(tokens).unwrap();
    println!("=== [parse tree] ===\n{:#?}", tree);
    let insns = tree_to_instructions(tree).unwrap();
    println!("=== [instructions] ===\n{:#?}", insns);

    println!("final vm state: {:#?}", run(insns).unwrap().debug());
}
