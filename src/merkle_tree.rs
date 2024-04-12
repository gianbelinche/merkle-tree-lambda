use super::hasher;

#[derive(Debug,PartialEq,Eq)]
pub enum CustomError{
    DataNotInTree
}
pub struct MerkleTree {
    leafs: Vec<String>
}

// Represents the necessary elements to prove that something is in the Merkle Tree
#[derive(PartialEq,Eq,Debug)]
pub struct Proof {
    index: usize,
    proofs: Vec<String>,
    root: String,
    leaf: String
}

// Returns a new merkle tree based on string data
pub fn construct_merkle_tree(data: Vec<String>) -> MerkleTree{
    let mut leafs: Vec<String> = vec![];
    for value in data {
        leafs.push(hasher::hash(value));
    }

    MerkleTree{leafs}
}

// Calculates de root of the merkle tree
pub fn calculate_root(tree: &MerkleTree) -> String{
    let level = get_next_level(&tree.leafs);

    if level.len() == 1 {
        level[0].to_string()
    } else {
        calculate_root(&MerkleTree{leafs:level})
    }
}

// Generates the necessary proof to know if an element is in the tree
pub fn generate_proof(tree: &MerkleTree, data: String) -> Result<Proof,CustomError> {
    let leaf = hasher::hash(data);
    let root = calculate_root(&tree);
    let index = match tree.leafs.iter().position(|r| *r == leaf) {
        Some(i) => i,
        None => {return Err(CustomError::DataNotInTree)}
    };

    let mut size = tree.leafs.len();
    let mut current_index = index;
    let mut proofs_indexes: Vec<usize> = vec![];
    while size > 1 {
        if current_index % 2 == 0 {
            if current_index + 1 >= size {
                proofs_indexes.push(current_index);
                size += 1;
            } else {
                proofs_indexes.push(current_index + 1);
            }
        } else {
            proofs_indexes.push(current_index - 1);
        }
        size = size / 2;
        current_index = current_index / 2;
    }
    let proofs = get_actual_proofs(tree,proofs_indexes);
    Ok(Proof{index,proofs,root,leaf})
}

// Given a proof, it verifies is the element inside it is in the merkle tree
pub fn verify_proof(tree: &MerkleTree, proof: &Proof) -> bool {
    let mut current_hash = tree.leafs[proof.index].to_string();
    let mut proof_index = proof.index;
    for hash in &proof.proofs {
        if proof_index % 2 == 0{ 
            current_hash = hasher::hash(current_hash + &hash);
        } else {
            current_hash = hasher::hash(hash.to_owned() + &current_hash);
        }
        proof_index = proof_index / 2;
    }
    current_hash == proof.root
}

// Given a list of indexes for each level of the tree, it returns the hashes of the elements in those indexes
fn get_actual_proofs(tree: &MerkleTree, indexes: Vec<usize>) -> Vec<String> {
    let mut proofs: Vec<String> = vec![];
    let mut tree_leafs = tree.leafs.clone();
    proofs.push(tree_leafs[indexes[0]].to_string());

    for index in indexes.iter().skip(1) {
        let level = get_next_level(&tree_leafs);
        proofs.push(level[*index].to_string());

        tree_leafs = level.clone();
    }

    proofs
}

// Returns the next level of the tree
fn get_next_level(tree_leafs: &Vec<String>) -> Vec<String> {
    let mut level: Vec<String> = vec![];
    let mut i = 0;
    while i < tree_leafs.len() {
        let left_leaf = &tree_leafs[i];
        let right_leaf;
        if i + 1 >= tree_leafs.len() {
            right_leaf = &tree_leafs[i];
        } else {
            right_leaf = &tree_leafs[i+1];
        }
        level.push(hasher::hash(left_leaf.to_owned() + &*right_leaf));
        i += 2;
    }
    level
}

#[cfg(test)]
mod tests {
    use crate::merkle_tree::calculate_root;

    use super::{construct_merkle_tree, generate_proof, verify_proof, Proof};

    use crate::merkle_tree::CustomError;

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

        assert_eq!{calculated_root,calculate_root(&merkle_tree)};
    }

    #[test]
    fn test_generate_proofs() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9".to_string();
        let leaf = "ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8".to_string();
        let proofs = vec![
            "036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da".to_string(),
            "1263c6ae9a0abc50f3516d6f4c60fc4d42b3366c93210b63d12a135784ac7b83".to_string()
        ];
        let actual_proof = Proof{index:1,root,leaf,proofs};

        let proof = generate_proof(&tree,"leaf2".to_string()).unwrap();

        assert_eq!(actual_proof,proof);
    }

    #[test]
    fn test_proof_not_existing() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let tree = construct_merkle_tree(leafs);

        assert_eq!(generate_proof(&tree, "leaf5".to_string()),Err(CustomError::DataNotInTree));
    }

    #[test]
    fn test_generate_complex_proofs() {
        /*
            Merkle tree

            cf57ea8368f4092d5998e0fea16141614c29cab4b77336ebe4cd3e6223c1c936

            89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9 - f9a67772f15224ba92dc938ec5a2fc06cc91cd93327d014e909a2f111a5ab4fa

            a8ad19d0c66c907e56aa4334e8189f10f65c0edaa0498f77539379d58f10ca8f - 1263c6ae9a0abc50f3516d6f4c60fc4d42b3366c93210b63d12a135784ac7b83
            380c82b011c04c9df13aa5f1275ba39430bc9bbfbf67f915efd3462540610b5a - 864e9d2e2c24a32b1c9f4c485bf2301d88ba9b30b00a411329c5d322e622df13

            036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da - ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8 
            fdd1f2a1ec75fe968421a41d2282200de6bec6a21f81080a71b1053d9c0120f3 - 157c9118369926e028fa6cf8dfe68c750c1adbd7b0e4918c2b3a23fe4017c732 
            075b8504c98679ed33a097ef9b1466e8f4652142d4b19ab2c37fdf1668a65c86 - 5e4e9020e0ae47d1b353192e416c1873ec642cb2e75d68faa1be2665df6c3c48 
            aa5dba90c8178f75a5eb7bdb098f0cec525bb61e5ba2ad6dc5fa580f163efdc7 - 5b62254e0339ae673a4773d31a564007f04f6ccf4934746b49ada87734d04cab
        */
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string(),
                                      "leaf5".to_string(),"leaf6".to_string(),"leaf7".to_string(),"leaf8".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "cf57ea8368f4092d5998e0fea16141614c29cab4b77336ebe4cd3e6223c1c936".to_string();
        let leaf = "075b8504c98679ed33a097ef9b1466e8f4652142d4b19ab2c37fdf1668a65c86".to_string();
        let proofs = vec![
            "5e4e9020e0ae47d1b353192e416c1873ec642cb2e75d68faa1be2665df6c3c48".to_string(),
            "864e9d2e2c24a32b1c9f4c485bf2301d88ba9b30b00a411329c5d322e622df13".to_string(),
            "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9".to_string()
        ];
        let actual_proof = Proof{index:4,root,leaf,proofs};

        let proof = generate_proof(&tree,"leaf5".to_string()).unwrap();

        assert_eq!(actual_proof,proof);
    }

    #[test]
    fn test_correct_verify_proof() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9".to_string();
        let leaf = "ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8".to_string();
        let proofs = vec![
            "036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da".to_string(),
            "1263c6ae9a0abc50f3516d6f4c60fc4d42b3366c93210b63d12a135784ac7b83".to_string()
        ];
        let index = 1;

        let proof = Proof {index, proofs, root, leaf};


        assert!(verify_proof(&tree, &proof));
    }

    #[test]
    fn test_wrong_verify_proof_1() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9".to_string();
        let leaf = "ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8".to_string();
        let proofs = vec![
            "bad_proof".to_string(),
            "1263c6ae9a0abc50f3516d6f4c60fc4d42b3366c93210b63d12a135784ac7b83".to_string()
        ];
        let index = 1;

        let proof = Proof {index, proofs, root, leaf};


        assert!(!verify_proof(&tree, &proof));
    }

    #[test]
    fn test_wrong_verify_proof_2() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9".to_string();
        let leaf = "ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8".to_string();
        let proofs = vec![
            "036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da".to_string(),
            "wrong_proof".to_string()
        ];
        let index = 1;

        let proof = Proof {index, proofs, root, leaf};


        assert!(!verify_proof(&tree, &proof));
    }

    #[test]
    fn test_wrong_verify_proof_3() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "wrong_root".to_string();
        let leaf = "ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8".to_string();
        let proofs = vec![
            "036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da".to_string(),
            "1263c6ae9a0abc50f3516d6f4c60fc4d42b3366c93210b63d12a135784ac7b83".to_string()
        ];
        let index = 1;

        let proof = Proof {index, proofs, root, leaf};


        assert!(!verify_proof(&tree, &proof));
    }

    #[test]
    fn test_calculate_root_not_power_of_two() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string()];

        let calculated_root = "89e9c0c63b9dd1f3c79a58ff99936a79b282dceb7008ed43d6ee36c8e8ded370";

        let merkle_tree = construct_merkle_tree(leafs);

        assert_eq!{calculated_root,calculate_root(&merkle_tree)};
    }

    #[test]
    fn test_calculate_root_not_power_of_two_2() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string(),"leaf5".to_string()];

        let calculated_root = "b81447ac40836eaa9b78cd024952d096d6aa8f6e6e39415f021a7d85097b4d54";

        let merkle_tree = construct_merkle_tree(leafs);

        assert_eq!{calculated_root,calculate_root(&merkle_tree)};
    }

    #[test]
    fn test_generate_proofs_not_power_of_2() {
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "89e9c0c63b9dd1f3c79a58ff99936a79b282dceb7008ed43d6ee36c8e8ded370".to_string();
        let leaf = "fdd1f2a1ec75fe968421a41d2282200de6bec6a21f81080a71b1053d9c0120f3".to_string();
        let proofs = vec![
            "fdd1f2a1ec75fe968421a41d2282200de6bec6a21f81080a71b1053d9c0120f3".to_string(),
            "a8ad19d0c66c907e56aa4334e8189f10f65c0edaa0498f77539379d58f10ca8f".to_string()
        ];
        let actual_proof = Proof{index:2,root,leaf,proofs};

        let proof = generate_proof(&tree,"leaf3".to_string()).unwrap();
        assert_eq!(actual_proof,proof);
    }

    #[test]
    fn test_generate_complex_proofs_not_power_of_two() {
        /*
            Merkle tree

            b81447ac40836eaa9b78cd024952d096d6aa8f6e6e39415f021a7d85097b4d54

            89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9 - 588d0604265a2dcf9136210d90eb8b6f93df99e7d845f7e0d687660deac71bda

            a8ad19d0c66c907e56aa4334e8189f10f65c0edaa0498f77539379d58f10ca8f - 1263c6ae9a0abc50f3516d6f4c60fc4d42b3366c93210b63d12a135784ac7b83
            4ff7c87454e2b312240e45e24281f77cdd7940ab628f4f8e11c07ef9d9f7bce6 - 4ff7c87454e2b312240e45e24281f77cdd7940ab628f4f8e11c07ef9d9f7bce6

            036491cc10808eeb0ff717314df6f19ba2e232d04d5f039f6fa382cae41641da - ba620d61dac4ddf2d7905722b259b0bd34ec4d37c5796d9a22537c54b3f972d8 
            fdd1f2a1ec75fe968421a41d2282200de6bec6a21f81080a71b1053d9c0120f3 - 157c9118369926e028fa6cf8dfe68c750c1adbd7b0e4918c2b3a23fe4017c732 
            075b8504c98679ed33a097ef9b1466e8f4652142d4b19ab2c37fdf1668a65c86 - 075b8504c98679ed33a097ef9b1466e8f4652142d4b19ab2c37fdf1668a65c86 
        */
        let leafs = vec!["leaf1".to_string(),"leaf2".to_string(),"leaf3".to_string(),"leaf4".to_string(),
                                      "leaf5".to_string()];

        let tree = construct_merkle_tree(leafs);

        let root = "b81447ac40836eaa9b78cd024952d096d6aa8f6e6e39415f021a7d85097b4d54".to_string();
        let leaf = "075b8504c98679ed33a097ef9b1466e8f4652142d4b19ab2c37fdf1668a65c86".to_string();
        let proofs = vec![
            "075b8504c98679ed33a097ef9b1466e8f4652142d4b19ab2c37fdf1668a65c86".to_string(),
            "4ff7c87454e2b312240e45e24281f77cdd7940ab628f4f8e11c07ef9d9f7bce6".to_string(),
            "89427e54728f5c7ec0aa205542861239c41f8b99404e383efeeef7ce752065e9".to_string()
        ];
        let actual_proof = Proof{index:4,root,leaf,proofs};

        let proof = generate_proof(&tree,"leaf5".to_string()).unwrap();

        assert_eq!(actual_proof,proof);
    }
}
