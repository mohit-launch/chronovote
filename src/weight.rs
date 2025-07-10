use crate::{decay::{calculate_weight, DecayModel}, voter};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use rust_decimal::prelude::ToPrimitive;


#[derive(Debug, Clone)]
pub struct WeightedVote {
    pub voter_id: String,
    pub vote_time: DateTime<Utc>,
    pub orig_weight: Decimal,
    pub decay_model: DecayModel,
    pub reputation_bonus: Decimal,
}
impl WeightedVote {
    pub fn effective_weight(&self, vote_start: DateTime<Utc>, now: DateTime<Utc>) -> Decimal {
        let decayed = Decimal::from_f64_retain(calculate_weight(
            self.orig_weight.to_f64().unwrap_or(0.0),
            vote_start,
            now,
            self.decay_model.clone(),
        ))
        .unwrap_or(dec!(0.0));
        decayed * (dec!(1.0) + self.reputation_bonus)
    }
}

#[derive(Default)]
pub struct WeightEngine {
    pub cache: HashMap<String, Decimal>,
    pub history: Vec<(String, Decimal, DateTime<Utc>)>,
    pub reputation: HashMap<String, Decimal>,
}

impl WeightEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calculate_and_cache(
        &mut self,
        vote: &WeightedVote,
        vote_start: &DateTime<Utc>,
        now: DateTime<Utc>,
    ) -> Decimal {
        let weight = vote.effective_weight(*vote_start, now);
        self.cache.insert(vote.voter_id.clone(), weight);
        self.history.push((vote.voter_id.clone(), weight, now));
        weight
    }

    pub fn batch_updates(
        &mut self,
        vote: &[WeightedVote],
        vote_start: &DateTime<Utc>,
        now: DateTime<Utc>,
    ) -> HashMap<String, Decimal> {
          let mut results = HashMap::new();
        for vote in vote {
            let weight = self.calculate_and_cache(vote, vote_start, now);
            results.insert(vote.voter_id.clone(), weight);
        }
        results
    }
    pub fn set_reputation(&mut self,voter_id:&String,bonus:Decimal){
        self.reputation.insert(voter_id.to_string(),bonus);
    }
    pub fn get_cached_weight(&self,voter_id:&str)->Option<Decimal>{
        self.cache.get(voter_id).cloned()
    }
    pub fn get_history(&self)->&Vec<(String,Decimal,DateTime<Utc>)>{
        &self.history
    }
}

