use std::time::Duration;

use super::StopStrategy;

pub struct StopNever;

impl StopStrategy for StopNever {
    fn should_stop(&self, _: usize, _: Duration) -> bool {
        false
    }
}

pub type StopAfterAttempts = usize;

impl StopStrategy for StopAfterAttempts {
    fn should_stop(&self, attempt: usize, _: Duration) -> bool {
        attempt >= *self
    }
}

pub type StopAfterDelay = Duration;

impl StopStrategy for StopAfterDelay {
    fn should_stop(&self, _: usize, elapsed: Duration) -> bool {
        elapsed > *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_stop_after_attempts() {
        let s = StopAfterAttempts::from(3u16);
        let elpased = Duration::from_secs(1);

        for attempt in 0..3 {
            assert!(!s.should_stop(attempt, elpased));
        }

        assert!(s.should_stop(3, elpased));
    }

    #[test]
    fn test_stop_after_delay() {
        let s = StopAfterDelay::from(Duration::from_secs(5));

        assert!(!s.should_stop(1, Duration::from_millis(100)));
        assert!(!s.should_stop(1, Duration::from_millis(1000)));
        assert!(s.should_stop(1, Duration::from_millis(10000)));
    }
}
