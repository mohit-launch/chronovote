use chrono::{DateTime,Utc};
use serde::{Serialize,Deserialize};
use ed25519_dalek::{Signature,Signer,Verifier,SigningKey,VerifyingKey};
use rand::rngs::OsRng;
use crate::decay::{DecayModel,calculate_weight};

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
        serde_json::to_vec(self).unwrap()
    }
    pub fn sign(&self,signing_key:&SigningKey)->SignedVote{
        let msg=self.to_bytes();
        let signature=signing_key.sign(&msg);
        SignedVote{
            vote:self.clone(),
            signature,
            public_key:signing_key.verifying_key(),
        }
    }
}

#[derive(Debug,Clone)]
pub struct SignedVote{
   pub vote:Vote,
   pub signature:Signature,
   pub public_key:VerifyingKey,
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

    pub fn compute_weight(&self,current_time:DateTime<Utc>,decay_model:DecayModel)->f64{
        calculate_weight(
            self.vote.vote_weight,
            self.vote.vote_time, // This is the vote's timestamp
            current_time,       // This is the current time
            decay_model,
       )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{SigningKey,VerifyingKey};
    use chrono::Utc;
    use rand::rngs::OsRng;

    #[test]
    fn test_vote_serialization() {
        let vote = Vote {
            voter_id: "Alice".into(),
            validator_id: "Validator1".into(),
            vote_time: Utc::now(),
            vote_weight: 1.0,
        };

        let bytes = vote.to_bytes();
        assert!(!bytes.is_empty(), "Vote did not serialize correctly");
    }

    #[test]
    fn test_vote_signing_and_verification() {
        let mut csprng = OsRng; 
        let signing_key: SigningKey = SigningKey::generate(&mut csprng); 
        let vote = Vote {
            voter_id: "Bob".into(),
            validator_id: "Validator2".into(),
            vote_time: Utc::now(),
            vote_weight: 1.5,
        };

        let signed_vote = vote.sign(&signing_key);
        let msg = vote.to_bytes();

        // Use the public key to verify the signature
        assert!(
            signed_vote.public_key.verify(&msg, &signed_vote.signature).is_ok(),
            "Signature verification failed"
        );
    }
}
