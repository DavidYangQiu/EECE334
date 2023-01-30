//! This module implements the blockchain.
//! 
//! You need to implement the `Blockchain` struct and its methods.

use crate::block::Block;
use crate::crypto::hash::{H256, Hashable};
use std::collections::HashMap;

pub struct Blockchain {
    //hash_to_block: a HashMap that maps the hash of each block to its corresponding Block struct.
    hash_to_block: HashMap<H256, Block>,
    //longest_chain: a vector of hashes of blocks along the longest chain, starting from the genesis block to the tip.
    longest_chain: Vec<H256>,
}

impl Blockchain {
    pub fn new() -> Self {
        //Create a new blockchain, only containing the genesis block
        let genesis = Block::genesis();
        let mut blockchain = Blockchain {
            hash_to_block: HashMap::new(),
            longest_chain: vec![genesis.hash()],
        };
        blockchain.hash_to_block.insert(genesis.hash(), genesis);
        blockchain
    }

    pub fn insert(&mut self, block: &Block) {
        let parent_hash = block.header.parent;
        let block_hash = block.hash();
        let parent_block = self.hash_to_block.get(&parent_hash).unwrap();
        let parent_index = self.longest_chain.iter().position(|&x| x == parent_hash).unwrap();
        if parent_index == self.longest_chain.len() - 1 {
            self.hash_to_block.insert(block_hash, block.clone());
            self.longest_chain.push(block_hash);
        } else {
            let new_longest_chain = self.longest_chain[0..=parent_index].to_vec();
            new_longest_chain.push(block_hash);
            self.hash_to_block = self.hash_to_block.iter().filter(|(k, _)| new_longest_chain.contains(k)).map(|(k, v)| (*k, v.clone())).collect();
            self.longest_chain = new_longest_chain;
        }
    }

    pub fn tip(&self) -> H256 {
        self.longest_chain[self.longest_chain.len() - 1]
    }

    #[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        self.longest_chain.clone()
    }
}


#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::block::test::generate_random_block;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
    }

    #[test]
    fn mp1_insert_chain() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let mut block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());
        for _ in 0..50 {
            let h = block.hash();
            block = generate_random_block(&h);
            blockchain.insert(&block);
            assert_eq!(blockchain.tip(), block.hash());
        }
    }

    #[test]
    fn mp1_insert_3_fork_and_back() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block_1 = generate_random_block(&genesis_hash);
        blockchain.insert(&block_1);
        assert_eq!(blockchain.tip(), block_1.hash());
        let block_2 = generate_random_block(&block_1.hash());
        blockchain.insert(&block_2);
        assert_eq!(blockchain.tip(), block_2.hash());
        let block_3 = generate_random_block(&block_2.hash());
        blockchain.insert(&block_3);
        assert_eq!(blockchain.tip(), block_3.hash());
        let fork_block_1 = generate_random_block(&block_2.hash());
        blockchain.insert(&fork_block_1);
        assert_eq!(blockchain.tip(), block_3.hash());
        let fork_block_2 = generate_random_block(&fork_block_1.hash());
        blockchain.insert(&fork_block_2);
        assert_eq!(blockchain.tip(), fork_block_2.hash());
        let block_4 = generate_random_block(&block_3.hash());
        blockchain.insert(&block_4);
        assert_eq!(blockchain.tip(), fork_block_2.hash());
        let block_5 = generate_random_block(&block_4.hash());
        blockchain.insert(&block_5);
        assert_eq!(blockchain.tip(), block_5.hash());
    }

}
