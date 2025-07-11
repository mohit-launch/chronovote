use chrono::{DateTime,Utc};

#[derive(Debug,Clone)]
pub enum ProgressionProfile{
    Conservative, //slow increase
    Aggresive, //fast increase
    Adaptive,  //based on participation
}

#[derive(Debug,Clone)]
pub struct ThresholdRequirement{
    pub min_percentage: f64,
    pub max_abs:usize,  //scheduling purpose
}

#[derive(Debug,Clone)]
pub enum Proposaltype{
    Normal, // less threshold
    Critical, // level of threshold increases
    Emergency, //max threshold i.e, 90%
}

#[derive(Debug)]
pub struct ProposalHistory{
    pub vote_time:DateTime<Utc>,
    pub total_vote:usize,
    pub yes_votes:usize,
    pub threshold_passed:bool,
}


impl ThresholdRequirement{
    pub fn is_met(&self,yes_votes:usize,total_vote:usize)->bool{
        if total_vote==0{
            return false;
        }
        let percentage=yes_votes as f64/total_vote as f64;
        percentage>=self.min_percentage &&yes_votes>=self.max_abs
    }
}
pub fn threshold_at(profile:&ProgressionProfile,elapsed_time:u64,participation:f64)->f64{
    match profile{
        ProgressionProfile::Conservative=>0.51+0.01*(elapsed_time as f64/300.0),
        ProgressionProfile::Aggresive=>0.51+0.02*(elapsed_time as f64/60.0),
        ProgressionProfile::Adaptive=>{
            if participation<0.30{
                 0.70
            }else{
                0.55+0.01*(elapsed_time as f64/120.0)
            }
        }
    }
    .min(0.90)
}

pub fn scheduled_base_threshold(now:u32)->f64{
    match now {
        0..=6=>0.70,
        7..=18=>0.55,
        _=>0.60
    }
}

pub fn requirement_for_type(p:Proposaltype)->ThresholdRequirement{
    match p{
        Proposaltype::Normal=>ThresholdRequirement{
            min_percentage:0.51,
            max_abs:5,
        },
         Proposaltype::Critical=>ThresholdRequirement{
            min_percentage:0.80,
            max_abs:15,
        },
         Proposaltype::Emergency=>ThresholdRequirement{
            min_percentage:0.90,
            max_abs:0,
        },

    }
}

pub fn recommend_profile(history:&[ProposalHistory])->ProgressionProfile{
    if history.is_empty(){
        return ProgressionProfile::Conservative;
    }
    let avg_participation:f64=history
        .iter()
        .map(|h|h.total_vote as f64)
        .sum::<f64>()
        /history.len() as f64;
    
    if avg_participation<5.0{
        ProgressionProfile::Aggresive
    }else{
        ProgressionProfile::Conservative
    }
}

