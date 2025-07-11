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
        hasher.update(&self.prev_hash);
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
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i-1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.prev_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}