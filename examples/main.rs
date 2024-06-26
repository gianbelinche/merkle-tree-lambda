use merkle_tree_lambda::merkle_tree::{construct_merkle_tree,verify_proof};

fn main() {
    let mut merkle_tree = construct_merkle_tree(vec!["leaf1".to_string(), "leaf2".to_string()]);
    merkle_tree.add_element("leaf3".to_string());
    merkle_tree.remove_element("leaf2".to_string());

    let proofs = merkle_tree.generate_proof("leaf1".to_string()).unwrap();
    verify_proof(&proofs);
    println!("{:#?}", proofs);
}
