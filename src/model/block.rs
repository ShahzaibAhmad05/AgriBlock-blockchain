use chrono::prelude::*;
use ethereum_types::U256;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use super::Transaction;

pub type BlockHash = U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: u64,
        nonce: u64,
        previous_hash: BlockHash,
        transactions: Vec<Transaction>,
    ) -> Block {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp_millis(),
            nonce,
            previous_hash,
            hash: BlockHash::default(),
            transactions,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> BlockHash {
        let mut hashable_data = self.clone();
        hashable_data.hash = BlockHash::default();
        let serialized = serde_json::to_string(&hashable_data).unwrap();

        // SHA-256 using sha2 crate
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let result = hasher.finalize();

        // Convert to U256 - using from_big_endian
        U256::from_big_endian(result.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::test_util::{alice, bob};

    #[test]
    fn should_create_block_with_transactions() {
        let tx = create_test_transaction();
        let transactions = vec![tx.clone()];
        let previous_hash = BlockHash::from(12345);

        let block = Block::new(1, 0, previous_hash, transactions.clone());

        assert_eq!(block.index, 1);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.previous_hash, previous_hash);
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.transactions[0].batch_id, tx.batch_id);
    }

    #[test]
    fn should_create_block_without_transactions() {
        let block = Block::new(0, 0, BlockHash::default(), Vec::new());

        assert_eq!(block.index, 0);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.previous_hash, BlockHash::default());
        assert!(block.transactions.is_empty());
    }

    #[test]
    fn should_calculate_hash() {
        let block = Block::new(1, 100, BlockHash::from(999), Vec::new());
        let calculated_hash = block.calculate_hash();

        assert_ne!(calculated_hash, BlockHash::default());
        assert_eq!(block.hash, calculated_hash);
    }

    #[test]
    fn should_calculate_different_hash_for_different_nonce() {
        let previous_hash = BlockHash::from(12345);

        let block1 = Block::new(1, 0, previous_hash, Vec::new());
        let block2 = Block::new(1, 1, previous_hash, Vec::new());

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn should_calculate_different_hash_for_different_index() {
        let previous_hash = BlockHash::from(12345);

        let block1 = Block::new(1, 0, previous_hash, Vec::new());
        let block2 = Block::new(2, 0, previous_hash, Vec::new());

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn should_calculate_different_hash_for_different_previous_hash() {
        let block1 = Block::new(1, 0, BlockHash::from(111), Vec::new());
        let block2 = Block::new(1, 0, BlockHash::from(222), Vec::new());

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn should_calculate_different_hash_for_different_transactions() {
        let tx1 = create_test_transaction();
        let mut tx2 = tx1.clone();
        tx2.batch_id = "DIFFERENT-BATCH".to_string();

        let previous_hash = BlockHash::from(12345);
        let block1 = Block::new(1, 0, previous_hash, vec![tx1]);
        let block2 = Block::new(1, 0, previous_hash, vec![tx2]);

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn should_recalculate_hash_correctly() {
        let mut block = Block::new(1, 0, BlockHash::from(999), Vec::new());
        let original_hash = block.hash;

        // Manually change the hash to something invalid
        block.hash = BlockHash::from(111);

        // Recalculate should return the original hash
        let recalculated_hash = block.calculate_hash();
        assert_eq!(recalculated_hash, original_hash);
    }

    #[test]
    fn should_clone_block() {
        let tx = create_test_transaction();
        let block1 = Block::new(1, 100, BlockHash::from(999), vec![tx]);
        let block2 = block1.clone();

        assert_eq!(block1.index, block2.index);
        assert_eq!(block1.nonce, block2.nonce);
        assert_eq!(block1.hash, block2.hash);
        assert_eq!(block1.previous_hash, block2.previous_hash);
        assert_eq!(block1.timestamp, block2.timestamp);
        assert_eq!(block1.transactions.len(), block2.transactions.len());
    }

    #[test]
    fn should_serialize_to_json() {
        let tx = create_test_transaction();
        let block = Block::new(1, 42, BlockHash::from(999), vec![tx]);

        let json = serde_json::to_string(&block).unwrap();
        assert!(json.contains("\"index\":1"));
        assert!(json.contains("\"nonce\":42"));
    }

    #[test]
    fn should_deserialize_from_json() {
        let tx = create_test_transaction();
        let original_block = Block::new(5, 123, BlockHash::from(456), vec![tx]);

        let json = serde_json::to_string(&original_block).unwrap();
        let deserialized_block: Block = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized_block.index, original_block.index);
        assert_eq!(deserialized_block.nonce, original_block.nonce);
        assert_eq!(deserialized_block.hash, original_block.hash);
        assert_eq!(deserialized_block.previous_hash, original_block.previous_hash);
    }

    #[test]
    fn should_have_consistent_hash_after_serialization() {
        let block1 = Block::new(1, 0, BlockHash::from(999), Vec::new());

        // Serialize and deserialize
        let json = serde_json::to_string(&block1).unwrap();
        let block2: Block = serde_json::from_str(&json).unwrap();

        // Hash should be the same
        assert_eq!(block1.hash, block2.hash);

        // Recalculating hash should give the same result
        assert_eq!(block2.calculate_hash(), block1.hash);
    }

    #[test]
    fn should_handle_multiple_transactions() {
        let tx1 = create_test_transaction();
        let mut tx2 = tx1.clone();
        tx2.batch_id = "WHEAT-002".to_string();
        let mut tx3 = tx1.clone();
        tx3.batch_id = "WHEAT-003".to_string();

        let block = Block::new(1, 0, BlockHash::default(), vec![tx1, tx2, tx3]);

        assert_eq!(block.transactions.len(), 3);
        assert_eq!(block.transactions[0].batch_id, "WHEAT-001");
        assert_eq!(block.transactions[1].batch_id, "WHEAT-002");
        assert_eq!(block.transactions[2].batch_id, "WHEAT-003");
    }

    fn create_test_transaction() -> Transaction {
        Transaction {
            sender: alice(),
            recipient: bob(),
            data: "Test harvest data".to_string(),
            batch_id: "WHEAT-001".to_string(),
            event_type: "HARVEST".to_string(),
        }
    }
}
