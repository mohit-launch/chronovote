use chrono::prelude::*;
use chrono::*;
use serde::{Serialize,Deserialize};
use ed25519_dalek::{Signature,PublicKey,Keypair,Signer,Validator};
use crate::Decay::{DecayModel,calculate_weight};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Vote {
   pub voter_id: String,
   pub validator_id: String,
   pub vote_time: DateTime<Utc>,
   pub vote_weight: f64,

}

impl Vote{
    //to verify the vote,we conver this to bytes
    pub fn to_bytes(&self)->Vec<u8>{
        serde_json::to_vec(self).unwrap();
    }
    pub fn sign(&self,keypair:&Keypair)->SignedVote{
        let msg=self.to_bytes();
        let signature=keypair.sign(&msg);
        SignedVote{
            vote:self,
            signature,
            public_key:Keypair.public,
        }
    }
}

#[derive(Debug,Clone)]
pub struct SignedVote{
   pub vote:Vote,
   pub signature:Signature,
   pub public_key:PublicKey,
}

impl SignedVote{
    //to verify the signed vote, we need to check if the signature is valid
    //using the public key of the validator
    pub fn verify(&self)->bool{
        let msg=self.vote.to_bytes();
        self.public_key
            .verify(&msg,&self.signature)
            .is_ok()
    }

    pub fn compute_weight(&self,vote_start:DateTime<Utc>,decay_model:DecayModel)->f64{
        calculate_weight{
            vote.Vote_weight,
            vote.Vote_time,
            vote_start,
            decay_model,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use ed25519_dalek::Keypair;

    #[test]
    fn test_vote_signature() {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);

        let vote = Vote {
            voter_id: "user".into(),
            validator_id: "val".into(),
            vote_time: Utc::now(),
            vote_weight: 100.0,
        };

        let signed = vote.sign(&keypair);
        assert!(signed.verify());
        println!("Signature verified!");
    }
}
