use super::hasher;
pub enum CustomError{
    DataNotInTree
}

pub struct MerkleTree {
    leafs: Vec<String>
}

pub struct Proof {
    index: usize,
    proofs: Vec<String>,
    root: String,
    leaf: String
}

pub fn construct_merkle_tree(data: Vec<String>) -> MerkleTree{
    let mut leafs: Vec<String> = vec![];
    for value in data {
        leafs.push(hasher::hash(value));
    }

    MerkleTree{leafs}
}

pub fn calculate_root(tree: MerkleTree) -> String{
    if !power_of_two(tree.leafs.len()) {
        panic!("Not power of 2") // This will be eliminated
    } 

    let mut level: Vec<String> = vec![];
    let mut i = 0;
    while i < tree.leafs.len() {
        let left_leaf = &tree.leafs[i];
        let right_leaf = &tree.leafs[i+1];
        level.push(hasher::hash(left_leaf.to_owned() + &*right_leaf));
        i += 2;
    }

    if level.len() == 1 {
        level[0].to_string()
    } else {
        calculate_root(MerkleTree{leafs:level})
    }
}

pub fn generate_proof(tree: MerkleTree, data: String) -> Result<Proof,CustomError> {
    let leaf = hasher::hash(data);
    let root = calculate_root(tree);
    let index = match tree.leafs.iter().position(|&r| r == leaf) {
        Some(i) => i,
        None => {return Err(CustomError::DataNotInTree)}
    };

    let mut size = tree.leafs.len();
    let mut current_index = index;
    let proofs: Vec<usize> = vec![];
    while size > 1 {
        if current_index % 2 == 0 {
            proofs.push(current_index + 1);
        } else {
            proofs.push(current_index - 1);
        }
        size = size / 2;
        current_index = current_index / 2;
    }
    // TODO: get actual hashes for proofs from indexes

    Ok(Proof{index,proofs,root,leaf})
}

fn power_of_two(x:usize) -> bool {
    (x & (x - 1)) == 0
}

#[cfg(test)]
mod tests {
    use crate::merkle_tree::calculate_root;

    use super::construct_merkle_tree;

    #[test]
    fn test_merkle_tree_construction() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let leafs_hashed = vec![
            "036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da",
            "ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8",
            "fdd1f2a1ec75fe968421a41d2282200de6bec6a21f81080a71b1053d9c0120f3",
            "157c9118369926e028fa6cf8dfe68c750c1adbd7b0e4918c2b3a23fe4017c732"
        ];

        assert_eq!{leafs_hashed,construct_merkle_tree(leafs).leafs};
    }

    #[test]
    fn test_calculate_root() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let calculated_root = "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9";

        let merkle_tree = construct_merkle_tree(leafs);

        assert_eq!{calculated_root,calculate_root(merkle_tree)};
    }
}
