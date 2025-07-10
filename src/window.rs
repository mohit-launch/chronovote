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
