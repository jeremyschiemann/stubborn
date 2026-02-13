use std::time::Duration;

pub mod stop;
pub mod wait;

pub trait WaitStrategy: Sized {
    fn wait_duration(&self, attempt: usize) -> Duration;
    fn add(self, other: impl WaitStrategy) -> impl WaitStrategy {
        WaitAdd {
            left: self,
            right: other,
        }
    }
}

pub trait StopStrategy: Sized {
    fn should_stop(&self, attempt: usize, elapsed: Duration) -> bool;

    fn or(self, other: impl StopStrategy) -> impl StopStrategy {
        StopOr {
            left: self,
            right: other,
        }
    }
}

struct WaitAdd<WL, WR>
where
    WL: WaitStrategy,
    WR: WaitStrategy,
{
    left: WL,
    right: WR,
}

impl<WL, WR> WaitStrategy for WaitAdd<WL, WR>
where
    WL: WaitStrategy,
    WR: WaitStrategy,
{
    fn wait_duration(&self, attempt: usize) -> Duration {
        self.left.wait_duration(attempt) + self.right.wait_duration(attempt)
    }
}

struct StopOr<SL, SR>
where
    SL: StopStrategy,
    SR: StopStrategy,
{
    left: SL,
    right: SR,
}

impl<SL, SR> StopStrategy for StopOr<SL, SR>
where
    SL: StopStrategy,
    SR: StopStrategy,
{
    fn should_stop(&self, attempt: usize, elapsed: Duration) -> bool {
        self.left.should_stop(attempt, elapsed) | self.right.should_stop(attempt, elapsed)
    }
}

#[cfg(test)]
mod tests {
    use crate::strategies::{
        stop::{StopAfterAttempts, StopAfterDelay},
        wait::{WaitExponential, WaitFixed},
    };

    use super::*;
    use std::time::Duration;

    #[test]
    fn test_stop_or() {
        let s = StopAfterAttempts::from(3u16).or(StopAfterDelay::from(Duration::from_secs(5)));

        assert!(!s.should_stop(2, Duration::from_secs(1)));
        assert!(s.should_stop(4, Duration::from_secs(1)));
        assert!(s.should_stop(2, Duration::from_secs(10)));
        assert!(s.should_stop(4, Duration::from_secs(10)));
    }

    #[test]
    fn test_wait_add() {
        let s = WaitExponential::new(Duration::from_secs(1), 2.0).add(WaitFixed::from_secs(1));

        assert_eq!(s.wait_duration(0), Duration::from_secs(2));
        assert_eq!(s.wait_duration(1), Duration::from_secs(3));
        assert_eq!(s.wait_duration(2), Duration::from_secs(5))
    }
}
