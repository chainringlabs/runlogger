use anyhow::{anyhow, Result};
use jiff::{
    civil::{Date, Time},
    Timestamp,
};

/// The time with any fraction of a second dropped.
pub fn whole_seconds(time: Time) -> Time {
    jiff::civil::time(time.hour(), time.minute(), time.second(), 0)
}

#[derive(Debug, Clone)]
pub struct Run {
    #[allow(dead_code)]
    pub id: Option<i64>,
    pub date: Date,
    pub time_started: Time,
    pub distance_miles: f64,
    pub note: Option<String>,
    pub created_at: Timestamp,
}

impl Run {
    // @spec QE-013
    pub fn new(date: Date, time: Time, distance: f64, note: Option<String>) -> Result<Self> {
        if !distance.is_finite() {
            return Err(anyhow!("Invalid distance. Enter a number (e.g., 3.5)"));
        }

        if distance <= 0.0 {
            return Err(anyhow!("Distance must be positive"));
        }

        if distance > 200.0 {
            return Err(anyhow!("Distance seems unrealistic (>200 miles)"));
        }

        Ok(Self {
            id: None,
            date,
            time_started: time,
            distance_miles: distance,
            note,
            created_at: Timestamp::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::{date, time};

    // @spec QE-013
    #[test]
    fn non_finite_distance_is_rejected() {
        let date = date(2026, 10, 1);
        let time = time(7, 0, 0, 0);
        for distance in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let err = Run::new(date, time, distance, None).unwrap_err();
            assert_eq!(
                err.to_string(),
                "Invalid distance. Enter a number (e.g., 3.5)"
            );
        }
    }
}
