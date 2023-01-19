use super::hash::{Hashable, H256};
use std::collections::VecDeque;
use ring::digest;
/// A Merkle tree.
#[derive(Debug, Default)]
pub struct MerkleTree {
    level:usize,
    tree_node: H256,
}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self where T: Hashable, {
        //unimplemented!()
        let mut num_data = data.len();
        let mut num_temp=0;
        let mut tree_node: Vec<H256> = Vec::new();
        let mut tree_queue: VecDeque<H256> = VecDeque::new();

        if num_data==0{
            let scope: [u8; 32] = [0; 32];
            tree_node.push(scope.into());

            return MerkleTree{level:0, tree_node:tree_node};
        }

        if num_data % 2 != 0 {
            tree_queue.push_back(data[num_data - 1].hash());
            num_data= num_data+1;
        }

        for t in data.into_iter() {
            tree_queue.push_back(t.hash());
        }

        let mut num_data_temp = num_data;

        while !tree_queue.is_empty() {
            let mut tree_hash = digest::Context::new(&digest::SHA256);

            let left_child = tree_queue.pop_front().unwrap();
            tree_node.push(left_child);
            num_temp = num_temp + 1;
            tree_hash.update(left_child.as_ref());

            let node_s = tree_queue.pop_front();
            if node_s == None {break; }

            let right_child = node_s.unwrap();
            tree_node.push(right_child);
            num_temp = num_temp + 1;
            tree_hash.update(right_child.as_ref());

            let node_parent: H256 = tree_hash.finish().into();
            tree_queue.push_back(node_parent);

            if num_temp != 2  && num_temp == num_data_temp {
                num_temp = 0;
                if (num_data_temp / 2) % 2 == 0 {
                    num_data_temp = num_data_temp / 2;
                } 
                else {
                    tree_queue.push_back(node_parent);
                    num_data_temp = num_data_temp / 2 + 1;
                }
            }
        }

        MerkleTree {
            level: num_data.
            tree_node: tree_node,
        }
    }

    pub fn root(&self) -> H256 {
        //unimplemented!()
        self.tree_node[self.tree_node.len() - 1]

    }

    /// Returns the Merkle Proof of data at index i
    pub fn proof(&self, index: usize) -> Vec<H256> {
        //unimplemented!()
    }
}

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {
   // unimplemented!()
   let mut idx = index;
   let mut node = *datum;
   let mut temp = digest::Context::new(&digest::SHA256);

   for t in 0..(proof.len()) {
    if idx % 2 != 0 {
        temp.update(proof[t].as_ref());
        temp.update(node.as_ref());
    } 
    if idx % 2 == 0{
        temp.update(node.as_ref());
        temp.update(proof[t].as_ref());
    }
    idx = idx / 2;
    node = temp.finish().into();
}
return node == *root;
}

#[cfg(test)]
mod tests {
    use crate::crypto::hash::H256;
    use super::*;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    #[test]
    fn root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }
}
