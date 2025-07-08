use chrono::{DateTime,Utc};

pub enum DecayModel{
    linear(f64),       //1% per minute
    exponential(f64), //0.1% per second
    //decay_factor is the percentage of weight to decay per step
    stepped{
        step_interval_secs: u64,
        decay_factor: f64,
    },
}

pub fn calculate_weight(vote_weight:f64,vote_start:DateTime<Utc>,vote_time:DateTime<Utc>,decay_model:DecayModel)->f64{
    let elapsed_time=(vote_time-vote_start).num_seconds() as f64;
    let min_weight=vote_weight*0.10 ;

    let decayed= match decay_model{
        DecayModel::linear(rate)=>{
            let decay=rate*elapsed_time;
            vote_weight=(1.0-decay);
        }
        DecayModel::exponential(rate)=>{
            decay=(-rate*elapsed_time).exp();
            vote_weight=vote_weight*decay;
        }
        DecayModel::stepped{step_interval_secs,decay_factor}=>{
            let steps=(elapsed_time/step_interval_secs as f64).floor();
            let decay=decay_factor*steps;
        vote_weight=vote_weight*(1.0-decay);
        }
    };
    decayed.max(min_weight)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_linear_decay() {
        let start = Utc::now();
        let vote_time = start + Duration::seconds(60); // 1 minute
        let weight = calculate_weight(100, start, vote_time, DecayModel::Linear(0.01));
        assert!(weight < 100.0 && weight >= 10.0);
    }

    #[test]
    fn test_exponential_decay() {
        let start = Utc::now();
        let vote_time = start + Duration::seconds(60);
        let weight = calculate_weight(100, start, vote_time, DecayModel::Exponential(0.01));
        assert!(weight < 100.0 && weight >= 10.0);
    }

    #[test]
    fn test_stepped_decay() {
        let start = Utc::now();
        let vote_time = start + Duration::seconds(180); // 3 steps
        let weight = calculate_weight(
            100,
            start,
            vote_time,
            DecayModel::Stepped {
                step_interval_secs: 60,
                decay_factor: 0.05,
            },
        );
        assert_eq!(weight, 100.0 * (1.0 - 0.15)); // 3 steps of 5% = 15%
    }

    #[test]
    fn test_weight_floor() {
        let start = Utc::now();
        let vote_time = start + Duration::seconds(99999); // very long time
        let weight = calculate_weight(100, start, vote_time, DecayModel::Linear(0.01));
        assert_eq!(weight, 10.0); // floor
    }
}
