use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub enum VotingWindow{
    Short, //5 min
    Medium, //30 min
    Long, // 2 hr
    Custom(Duration),
}

impl VotingWindow{
    pub fn duration(&self)->Duration{
        match self{
            VotingWindow::Short=>Duration::minutes(5),
            VotingWindow::Medium=>Duration::minutes(30),
            VotingWindow::Long=>Duration::hours(2),
            VotingWindow::Custom(dur)=>*dur,
        }
    }
}

#[derive(Debug)]
pub struct VotingSession{
    pub vote_start:DateTime<Utc>,
    pub voter_id:String,
    pub voting_window:VotingWindow,
    pub extended:bool,
}

impl VotingSession{
    pub fn end_time(&self)->DateTime<Utc>{
        self.vote_start+self.voting_window.duration()
    }

    pub fn has_expired(&self,now:DateTime<Utc>)->bool{
        now>self.end_time()
    }

    pub fn remaining_time(&self,now:DateTime<Utc>)->Duration{
        let end=self.end_time();
        if now>=end{
            Duration::zero()
        }
        else{
            end-now
        }
    }
    pub fn extend_if_possible(&mut self, extension: Duration) {
        if !self.extended {
            let current_duration = self.voting_window.duration();
            self.voting_window = VotingWindow::Custom(current_duration + extension);
            self.extended = true;
        }
    }
}

pub struct ProposalManager{
    pub proposals:HashMap<String,VotingSession>,
    pub grace_period:Duration,
}

impl ProposalManager{
    pub fn new(grace_period_secs:i64)->Self{
        Self{
            proposals:HashMap::new(),
            grace_period:Duration::seconds(grace_period_secs),
        }
    }

    pub fn add_proposal(&mut self, proposal_id: String, voter_id: String, voting_window: VotingWindow) {
        let session=VotingSession{
            vote_start: Utc::now(),
            voter_id: voter_id.to_string(),
            voting_window,
            extended: false,
        };
        self.proposals.insert(proposal_id.to_string(),session);
    }

    pub fn list_actives(&self,now:DateTime<Utc>)->Vec<(&String,&VotingSession)>{
        self.proposals
        .iter()
        .filter(|(_,session)| !session.has_expired(now+self.grace_period))
        .collect()
    }

    pub fn cleanup_expired(&mut self,now:DateTime<Utc>){
        self.proposals
        .retain(|_,session| !session.has_expired(now+self.grace_period));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn test_voting_window_durations() {
        assert_eq!(VotingWindow::Short.duration(), Duration::minutes(5));
        assert_eq!(VotingWindow::Medium.duration(), Duration::minutes(30));
        assert_eq!(VotingWindow::Long.duration(), Duration::hours(2));
        assert_eq!(
            VotingWindow::Custom(Duration::seconds(45)).duration(),
            Duration::seconds(45)
        );
    }

    #[test]
    fn test_session_end_time_and_expiration() {
        let start_time = Utc::now();
        let session = VotingSession {
            vote_start: start_time,
            voter_id: "voter1".to_string(),
            voting_window: VotingWindow::Short,
            extended: false,
        };

        let expected_end = start_time + Duration::minutes(5);
        assert_eq!(session.end_time(), expected_end);

        let now_before = start_time + Duration::minutes(3);
        assert!(!session.has_expired(now_before));

        let now_after = start_time + Duration::minutes(6);
        assert!(session.has_expired(now_after));
    }

    #[test]
    fn test_remaining_time_and_extension() {
        let mut session = VotingSession {
            vote_start: Utc::now(),
            voter_id: "voter1".to_string(),
            voting_window: VotingWindow::Short,
            extended: false,
        };

        let now = session.vote_start + Duration::minutes(2);
        let remaining = session.remaining_time(now);
        assert_eq!(remaining, Duration::minutes(3));

        session.extend_if_possible(Duration::minutes(5));
        assert!(session.extended);

        let new_end = session.vote_start + Duration::minutes(10); // 5 original + 5 extension
        assert_eq!(session.end_time(), new_end);
    }

    #[test]
    fn test_proposal_manager_add_and_list_active() {
        let mut manager = ProposalManager::new(60); // 60 seconds grace
        manager.add_proposal("p1".to_string(), "voterA".to_string(), VotingWindow::Short);

        let now = Utc::now();
        let actives = manager.list_actives(now);
        assert_eq!(actives.len(), 1);
        assert_eq!(actives[0].0, &"p1".to_string());
    }

    #[test]
    fn test_proposal_cleanup_expired() {
        let mut manager = ProposalManager::new(0); // No grace period

        // Add an expired proposal manually
        let past_time = Utc::now() - Duration::minutes(10);
        let expired_session = VotingSession {
            vote_start: past_time,
            voter_id: "voterX".to_string(),
            voting_window: VotingWindow::Short, // expired
            extended: false,
        };

        manager.proposals.insert("expired".to_string(), expired_session);

        // Add a still-active proposal
        manager.add_proposal("active".to_string(), "voterY".to_string(), VotingWindow::Long);

        manager.cleanup_expired(Utc::now());

        assert_eq!(manager.proposals.len(), 1);
        assert!(manager.proposals.contains_key("active"));
        assert!(!manager.proposals.contains_key("expired"));
    }
}
