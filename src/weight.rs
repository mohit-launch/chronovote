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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use rust_decimal_macros::dec;
    use crate::decay::DecayModel;

    fn sample_vote(
        voter_id: &str,
        orig_weight: Decimal,
        reputation_bonus: Decimal,
        decay_model: DecayModel,
    ) -> WeightedVote {
        WeightedVote {
            voter_id: voter_id.to_string(),
            vote_time: Utc::now(),
            orig_weight,
            decay_model,
            reputation_bonus,
        }
    }

    #[test]
    fn test_effective_weight_exponential_decay() {
        let vote_start = Utc::now();
        let now = vote_start + Duration::minutes(10);

        let vote = sample_vote("alice", dec!(1.0), dec!(0.1), DecayModel::Exponential(0.05));
        let weight = vote.effective_weight(vote_start, now);

        assert!(weight > dec!(0.0));
        assert!(weight <= dec!(1.1));
    }

    #[test]
    fn test_cache_and_history() {
        let vote_start = Utc::now();
        let now = vote_start + Duration::minutes(5);

        let vote = sample_vote("bob", dec!(2.0), dec!(0.2), DecayModel::Linear(0.02));
        let mut engine = WeightEngine::new();

        let weight = engine.calculate_and_cache(&vote, &vote_start, now);

        let cached = engine.get_cached_weight("bob").unwrap();
        assert_eq!(cached, weight);

        let history = engine.get_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].0, "bob");
    }

    #[test]
    fn test_batch_updates() {
        let vote_start = Utc::now();
        let now = vote_start + Duration::minutes(2);

        let votes = vec![
            sample_vote("alice", dec!(1.0), dec!(0.1), DecayModel::Linear(0.03)),
            sample_vote(
                "bob",
                dec!(2.0),
                dec!(0.0),
                DecayModel::Stepped {
                    step_interval_secs: 60,
                    decay_factor: 0.2,
                },
            ),
        ];

        let mut engine = WeightEngine::new();
        let results = engine.batch_updates(&votes, &vote_start, now);

        assert_eq!(results.len(), 2);
        assert!(results.contains_key("alice"));
        assert!(results.contains_key("bob"));
    }

    #[test]
    fn test_set_and_get_reputation() {
        let mut engine = WeightEngine::new();
        engine.set_reputation(&"carol".to_string(), dec!(0.3));

        let bonus = engine.reputation.get("carol").unwrap();
        assert_eq!(*bonus, dec!(0.3));
    }
}
