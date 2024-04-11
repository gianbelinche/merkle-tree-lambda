use crate::merkle_tree::generate_proof;

mod hasher;
mod merkle_tree;

fn main() {
    let merkle_tree = merkle_tree::construct_merkle_tree(vec!["leaf1".to_string(),"leaf2".to_string()]);

    let proofs = generate_proof(merkle_tree, "leaf1".to_string());
    println!("{:#?}",proofs);
}
