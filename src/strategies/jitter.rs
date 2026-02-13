use super::JitterStrategy;

pub struct NoJitter;

impl JitterStrategy for NoJitter {
    fn apply(&self, wait_duration: std::time::Duration) -> std::time::Duration {
        wait_duration
    }
}

#[cfg(feature = "random")]
mod random_jitter {
    use rand;
    use std::time::Duration;
    use crate::JitterStrategy;

    pub struct FullJitter;

    impl JitterStrategy for FullJitter {
        fn apply(&self, wait_duration: std::time::Duration) -> std::time::Duration {
            let millis = wait_duration.as_millis() as u64;

            if millis == 0 {
                return wait_duration;
            }

            let jittered = rand::random_range(0..millis);

            Duration::from_millis(jittered)
        }
    }

    pub struct EqualJitter;

    impl JitterStrategy for EqualJitter {
        fn apply(&self, wait_duration: std::time::Duration) -> std::time::Duration {
            let millis = wait_duration.as_millis() as u64;

            if millis == 0 {
                return wait_duration;
            }
            
            let half = millis / 2;
            let half_jittered = rand::random_range(0..half);

            Duration::from_millis(half + half_jittered)
        }
    }
}

#[cfg(feature = "random")]
pub use random_jitter::FullJitter;
