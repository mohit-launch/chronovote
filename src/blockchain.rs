use chrono::{DateTime,Utc};
use rand::seq::SliceRandom;
use rand::thread_rng;
use sha2::{Sha256,Digest};
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Block{
    pub index:usize,
    pub timestamp:DateTime<Utc>,
    pub data:String,
    pub prev_hash:String,
    pub hash:String,
}

impl Block{
    pub fn new(index:usize,timestamp:DateTime<Utc>,data:String,prev_hash:String)->Self{
        let mut block=Block{
            index,
            timestamp,
            data,
            prev_hash,
            hash:String::new(),
        };
        block.hash=block.calculate_hash();
        block
    }
    pub fn calculate_hash(&self)->String{
        let mut hasher=Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_rfc3339());
        hasher.update(&self.data);
        hasher.update(&self.hash);
        format!("{:x}",hasher.finalize())
    }
}

pub struct Blockchain{
    pub blocks:Vec<Block>,
}

impl Blockchain{
    pub fn new()->Self{
        let now=Utc::now();
       //lets add a genesis block
        let genesis = Block::new(0, now, String::from("Genesis Block"), String::from("0")); 
        Blockchain{
                blocks:vec![genesis],
        }
    }
    pub fn add_blocks(&mut self,data:String){
        let last_block=self.blocks.last().unwrap();
        let block=Block::new(
                self.blocks.len(),
            Utc::now(),
                       data,
            last_block.hash.clone(),
        );
        self.blocks.push(block)
    }

    pub fn is_valid(&self)->bool{
        for i in 1..self.blocks.len(){
            if self.blocks[i].prev_hash!=self.blocks[i-1].hash{
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_genesis_block() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.blocks[0].index, 0);
        assert_eq!(blockchain.blocks[0].data, "Genesis Block");
        assert_eq!(blockchain.blocks[0].prev_hash, "0");
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_blocks("Block 1".to_string());
        blockchain.add_blocks("Block 2".to_string());

        assert_eq!(blockchain.blocks.len(), 3);
        assert_eq!(blockchain.blocks[1].data, "Block 1");
        assert_eq!(blockchain.blocks[2].data, "Block 2");

        // Check if each block links to the previous one
        assert_eq!(blockchain.blocks[1].prev_hash, blockchain.blocks[0].hash);
        assert_eq!(blockchain.blocks[2].prev_hash, blockchain.blocks[1].hash);
    }

    #[test]
    fn test_chain_validity() {
        let mut blockchain = Blockchain::new();
        blockchain.add_blocks("Data A".to_string());
        blockchain.add_blocks("Data B".to_string());

        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_chain_tampering_detection() {
        let mut blockchain = Blockchain::new();
        blockchain.add_blocks("Alpha".to_string());
        blockchain.add_blocks("Beta".to_string());

        // Tamper with a block
        blockchain.blocks[1].data = "Tampered Data".to_string();

        // This should now be invalid because the hash chain is broken
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_hash_changes_on_data_change() {
        let timestamp = Utc::now();
        let block1 = Block::new(1, timestamp, "Original".to_string(), "0".to_string());
        let block2 = Block::new(1, timestamp, "Changed".to_string(), "0".to_string());

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn test_hash_is_consistent_for_same_data() {
        let timestamp = Utc::now();
        let block1 = Block::new(2, timestamp, "Same".to_string(), "abc".to_string());
        let block2 = Block::new(2, timestamp, "Same".to_string(), "abc".to_string());

        assert_eq!(block1.hash, block2.hash);
    }
}
