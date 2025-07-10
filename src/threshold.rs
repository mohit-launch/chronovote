use chrono::{DateTime,Utc};

pub enum ThresholdModel{
    Exponential(f64),
    Linear(f64),
    Sigmoid {steepness: f64,midpoint:f64},
    StepFn(Vec<(u64,f64)>),
}

// This enum represents the different types of thresholds that can be applied to emergency situations.
#[derive(Debug,Clone)]
pub enum ThresholdEmergency{
    Emergency(f64),
}

pub const MIN_THRESHOLD:f64=0.51;
pub const MAX_THRESHOLD:f64=0.90;

pub fn threshold_at(
    start_time: DateTime<Utc>,
    now:DateTime<Utc>,
    model:&ThresholdModel,
    override_mode:Option<ThresholdEmergency>,
)-> f64 {
    if let Some(ThresholdEmergency::Emergency(value))=override_mode{
        return value.max(MIN_THRESHOLD).min(MAX_THRESHOLD)
    }
    let elapsed_minutes=(now-start_time).num_minutes().max(0) as f64;

    let base=match model{
        ThresholdModel::Exponential(growth_rate)=>{
            MIN_THRESHOLD*(1.0+growth_rate).powf(elapsed_minutes)
        }
        ThresholdModel::Linear(slope)=>{
            MIN_THRESHOLD+slope*elapsed_minutes
        }
        ThresholdModel::Sigmoid{steepness,midpoint}=>{
            let x = elapsed_minutes;
            MIN_THRESHOLD + (MAX_THRESHOLD - MIN_THRESHOLD)
                / (1.0 + (-steepness * (x - midpoint)).exp())
        }
        ThresholdModel::StepFn(steps)=>{
              let mut threshold=MIN_THRESHOLD;
              for(time,value) in steps.iter(){
                if *time as f64<=elapsed_minutes{
                    threshold=*value;
                }
            }
            threshold

        }
    };
    base.min(MAX_THRESHOLD).max(MIN_THRESHOLD)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn test_linear_threshold_10_minutes() {
        let start = Utc::now();
        let now = start + Duration::minutes(10);
        let model = ThresholdModel::Linear(0.01);
        let result = threshold_at(start, now, &model, None);
        assert!((result - 0.61).abs() < 0.001, "Expected ~0.61, got {}", result);
    }

    #[test]
    fn test_exponential_threshold_growth() {
        let start = Utc::now();
        let now = start + Duration::minutes(5);
        let model = ThresholdModel::Exponential(0.05);
        let result = threshold_at(start, now, &model, None);
        assert!(result > MIN_THRESHOLD, "Should increase above minimum");
        assert!(result <= MAX_THRESHOLD, "Should not exceed maximum");
    }

    #[test]
    fn test_sigmoid_midpoint_behavior() {
        let start = Utc::now();
        let now = start + Duration::minutes(10);
        let model = ThresholdModel::Sigmoid {
            steepness: 1.0,
            midpoint: 10.0,
        };
        let result = threshold_at(start, now, &model, None);
        assert!(
            result > MIN_THRESHOLD && result < MAX_THRESHOLD,
            "Expected value in range, got {}",
            result
        );
    }

    #[test]
    fn test_step_function_threshold() {
        let start = Utc::now();
        let now = start + Duration::minutes(15);
        let model = ThresholdModel::StepFn(vec![
            (0, 0.51),
            (5, 0.55),
            (10, 0.65),
            (15, 0.75),
        ]);
        let result = threshold_at(start, now, &model, None);
        assert!((result - 0.75).abs() < 0.001, "Expected 0.75, got {}", result);
    }

    #[test]
    fn test_emergency_override_threshold() {
        let start = Utc::now();
        let now = start + Duration::minutes(50);
        let model = ThresholdModel::Linear(0.01);
        let result = threshold_at(start, now, &model, Some(ThresholdEmergency::Emergency(0.85)));
        assert!((result - 0.85).abs() < 0.001, "Emergency override failed");
    }

    #[test]
    fn test_threshold_min_max_clamp() {
        let start = Utc::now();
        let now = start + Duration::minutes(1000);
        let model = ThresholdModel::Linear(0.01);
        let result = threshold_at(start, now, &model, None);
        assert!(
            (result - MAX_THRESHOLD).abs() < 0.0001,
            "Expected clamped max threshold"
        );
    }
}
