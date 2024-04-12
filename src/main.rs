use crate::merkle_tree::{generate_proof, verify_proof};

mod hasher;
mod merkle_tree;

fn main() {
    let mut merkle_tree = merkle_tree::construct_merkle_tree(vec!["leaf1".to_string(),"leaf2".to_string()]);
    merkle_tree::add_element(&mut merkle_tree,"leaf3".to_string());
    merkle_tree::remove_element(&mut merkle_tree, "leaf2".to_string());

    let proofs = generate_proof(&merkle_tree, "leaf1".to_string()).unwrap();
    verify_proof(&merkle_tree, &proofs);
    println!("{:#?}",proofs);
}
