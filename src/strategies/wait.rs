use std::time::Duration;

use super::WaitStrategy;

pub type WaitFixed = Duration;

impl WaitStrategy for WaitFixed {
    fn wait_duration(&self, _: usize) -> WaitFixed {
        self.to_owned()
    }
}

pub struct WaitExponential {
    wait_duration_base: Duration,
    multiplier: f32,
}

impl WaitExponential {
    pub fn new(wait_duration_base: impl Into<Duration>, multiplier: impl Into<f32>) -> Self {
        Self {
            wait_duration_base: wait_duration_base.into(),
            multiplier: multiplier.into(),
        }
    }
}

impl WaitStrategy for WaitExponential {
    fn wait_duration(&self, attempt: usize) -> Duration {
        let factor = self.multiplier.powi(attempt as i32);
        self.wait_duration_base.mul_f32(factor)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::strategies::{
        wait::{WaitExponential, WaitFixed},
        WaitStrategy as _,
    };

    #[test]
    fn test_wait_fixed() {
        let s = WaitFixed::from_secs(1);

        for attempt in 0..3 {
            assert_eq!(s.wait_duration(attempt), Duration::from_secs(1));
        }
    }

    #[test]
    fn test_wait_exponential() {
        let s = WaitExponential::new(Duration::from_secs(1), 2.0);

        assert_eq!(s.wait_duration(0), Duration::from_secs(1));
        assert_eq!(s.wait_duration(1), Duration::from_secs(2));
        assert_eq!(s.wait_duration(2), Duration::from_secs(4));
    }
}
