mod hasher;
mod merkle_tree;

fn main() {
    let merkle_tree = merkle_tree::construct_merkle_tree(vec!["leaf1".to_string(),"leaf2".to_string()]);

    let root = merkle_tree::calculate_root(merkle_tree);

    println!("{}",root);
}
