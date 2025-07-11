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
           vote_weight * (-rate * elapsed_time).exp()
        }
        DecayModel::Stepped{step_interval_secs,decay_factor}=>{
            let steps=(elapsed_time/step_interval_secs as f64).floor();
            let decay=decay_factor*steps;
            vote_weight*(1.0-decay)
        }
    };
     f64::max(decayed, min_weight)

}

#[cfg(test)]
mod more_tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn test_zero_decay_linear() {
        let vote_weight = 50.0;
        let start = Utc::now();
        let later = start + Duration::minutes(5);
        let decay_model = DecayModel::Linear(0.0); // no decay

        let result = calculate_weight(vote_weight, start, later, decay_model);
        assert_eq!(result, vote_weight);
    }

    #[test]
    fn test_immediate_vote_exponential() {
        let vote_weight = 20.0;
        let start = Utc::now();
        let later = start; // no time has passed
        let decay_model = DecayModel::Exponential(0.01);

        let result = calculate_weight(vote_weight, start, later, decay_model);
        assert_eq!(result, vote_weight); // no decay yet
    }

    #[test]
    fn test_high_decay_linear_reaches_floor() {
        let vote_weight = 100.0;
        let start = Utc::now();
        let later = start + Duration::minutes(200); // long enough to decay below floor
        let decay_model = DecayModel::Linear(0.01); // 1% per minute

        let result = calculate_weight(vote_weight, start, later, decay_model);
        assert_eq!(result, 10.0); // floor is 10%
    }

    #[test]
    fn test_large_step_decay() {
        let vote_weight = 80.0;
        let start = Utc::now();
        let later = start + Duration::seconds(90); // 90 seconds elapsed
        let decay_model = DecayModel::Stepped {
            step_interval_secs: 30,
            decay_factor: 0.1, // 10% per step
        };

        // 90s / 30s = 3 steps → 30% decay → 80 * 0.7 = 56
        let result = calculate_weight(vote_weight, start, later, decay_model);
        assert!((result - 56.0).abs() < 0.001);
    }

    #[test]
    fn test_exponential_decay_heavy() {
        let vote_weight = 40.0;
        let start = Utc::now();
        let later = start + Duration::seconds(100);
        let decay_model = DecayModel::Exponential(0.1); // fast decay

        let result = calculate_weight(vote_weight, start, later, decay_model);
        // Should be significantly decayed, but still >= floor (40 * 0.1 = 4.0)
        assert!(result >= 4.0 && result <= 40.0);
    }
}
