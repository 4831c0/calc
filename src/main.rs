mod token;
mod tree;

use std::fs::File;
use crate::token::tokenize;
use crate::tree::tokens_to_tree;

fn main() {
    let file = File::open("input.txt").unwrap();
    let tokens = tokenize(file).unwrap();
    let tree = tokens_to_tree(tokens).unwrap();

    println!("{:#?}", tree);
}
