use chrono::{DateTime,Utc};

#[derive(Debug,Clone)]
pub enum DecayModel{
    Linear(f64),       //1% per minute
    Exponential(f64), //0.1% per second
    //decay_factor is the percentage of weight to decay per step
    Stepped{
        step_interval_secs: u64,
        decay_factor: f64,
    },
}

pub fn calculate_weight(vote_weight:f64,vote_start:DateTime<Utc>,vote_time:DateTime<Utc>,decay_model:DecayModel)->f64{
    let elapsed_time=(vote_time-vote_start).num_seconds() as f64;
    let min_weight=vote_weight*0.10 ;

    let decayed= match decay_model{
        DecayModel::Linear(rate)=>{
            let decay=rate*elapsed_time;
             vote_weight * (1.0 - decay)
        }
        DecayModel::Exponential(rate)=>{
           let decay=(-rate*elapsed_time).exp();
            vote_weight*decay
        }
        DecayModel::Stepped{step_interval_secs,decay_factor}=>{
            let steps=(elapsed_time/step_interval_secs as f64).floor();
            let decay=decay_factor*steps;
            vote_weight*(1.0-decay)
        }
    };
     f64::max(decayed, min_weight)

}

