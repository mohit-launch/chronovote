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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_threshold_requirement_met() {
        let req = ThresholdRequirement {
            min_percentage: 0.6,
            max_abs: 10,
        };

        assert!(req.is_met(12, 20)); // 60% and >= 10
        assert!(!req.is_met(5, 20)); // <10 yes_votes
        assert!(!req.is_met(12, 30)); // 40% < 60%
        assert!(!req.is_met(0, 0)); // total vote is zero
    }

    #[test]
    fn test_threshold_at_conservative() {
        let threshold = threshold_at(&ProgressionProfile::Conservative, 600, 0.5);
        // Should increase slowly: 0.51 + (600/300 * 0.01) = 0.53
        assert!((threshold - 0.53).abs() < 0.001);
    }

    #[test]
    fn test_threshold_at_aggressive() {
        let threshold = threshold_at(&ProgressionProfile::Aggresive, 120, 0.5);
        // 0.51 + 0.02 * (120 / 60) = 0.55
        assert!((threshold - 0.55).abs() < 0.001);
    }

    #[test]
    fn test_threshold_at_adaptive_low_participation() {
        let threshold = threshold_at(&ProgressionProfile::Adaptive, 100, 0.2);
        assert_eq!(threshold, 0.70);
    }

    #[test]
    fn test_threshold_at_adaptive_high_participation() {
        let threshold = threshold_at(&ProgressionProfile::Adaptive, 240, 0.5);
        // 0.55 + (240/120 * 0.01) = 0.57
        assert!((threshold - 0.57).abs() < 0.001);
    }

    #[test]
    fn test_scheduled_base_threshold() {
        assert_eq!(scheduled_base_threshold(3), 0.70); // Night
        assert_eq!(scheduled_base_threshold(12), 0.55); // Day
        assert_eq!(scheduled_base_threshold(20), 0.60); // Evening
    }

    #[test]
    fn test_requirement_for_type() {
        let normal = requirement_for_type(Proposaltype::Normal);
        assert_eq!(normal.min_percentage, 0.51);
        assert_eq!(normal.max_abs, 5);

        let critical = requirement_for_type(Proposaltype::Critical);
        assert_eq!(critical.min_percentage, 0.80);
        assert_eq!(critical.max_abs, 15);

        let emergency = requirement_for_type(Proposaltype::Emergency);
        assert_eq!(emergency.min_percentage, 0.90);
        assert_eq!(emergency.max_abs, 0);
    }

    #[test]
    fn test_recommend_profile_empty() {
        let history = vec![];
        let profile = recommend_profile(&history);
        match profile {
            ProgressionProfile::Conservative => (),
            _ => panic!("Expected Conservative for empty history"),
        }
    }

    #[test]
    fn test_recommend_profile_aggressive() {
        let history = vec![
            ProposalHistory {
                vote_time: Utc::now(),
                total_vote: 2,
                yes_votes: 1,
                threshold_passed: false,
            },
            ProposalHistory {
                vote_time: Utc::now(),
                total_vote: 3,
                yes_votes: 2,
                threshold_passed: true,
            },
        ];

        let profile = recommend_profile(&history);
        match profile {
            ProgressionProfile::Aggresive => (),
            _ => panic!("Expected Aggresive due to low average participation"),
        }
    }

    #[test]
    fn test_recommend_profile_conservative() {
        let history = vec![
            ProposalHistory {
                vote_time: Utc::now(),
                total_vote: 10,
                yes_votes: 8,
                threshold_passed: true,
            },
            ProposalHistory {
                vote_time: Utc::now(),
                total_vote: 12,
                yes_votes: 10,
                threshold_passed: true,
            },
        ];

        let profile = recommend_profile(&history);
        match profile {
            ProgressionProfile::Conservative => (),
            _ => panic!("Expected Conservative due to high average participation"),
        }
    }
}
