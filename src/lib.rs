use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::strategies::{
    stop::StopAfterAttempts,
    wait::{WaitExponential, WaitFixed},
    StopStrategy, WaitStrategy,
};

pub mod strategies;

pub struct Retry<S, W>
where
    S: StopStrategy,
    W: WaitStrategy,
{
    stop: S,
    wait: W,
}

pub struct RetryInfo {
    pub attempts: usize,
    pub elapsed: Duration,
}

impl Retry<StopAfterAttempts, WaitFixed> {
    pub fn basic() -> Self {
        Self {
            wait: WaitFixed::from_secs(1),
            stop: StopAfterAttempts::from(3usize),
        }
    }
}

impl Retry<StopAfterAttempts, WaitExponential> {
    pub fn exponential() -> Self {
        Self {
            stop: StopAfterAttempts::from(3usize),
            wait: WaitExponential::new(Duration::from_secs(1), 2.0),
        }
    }
}

impl<S, W> Retry<S, W>
where
    S: StopStrategy,
    W: WaitStrategy,
{
    pub fn stop(self, stop_strategy: impl StopStrategy) -> Retry<impl StopStrategy, W> {
        Retry {
            stop: stop_strategy,
            wait: self.wait,
        }
    }

    pub fn wait(self, wait_strategy: impl WaitStrategy) -> Retry<S, impl WaitStrategy> {
        Retry {
            stop: self.stop,
            wait: wait_strategy,
        }
    }

    pub fn call_with_info<F, T, E>(&self, mut f: F) -> (Result<T, E>, RetryInfo)
    where
        F: FnMut() -> Result<T, E>,
    {
        let mut attempt = 0usize;
        let started = Instant::now();

        loop {
            let result = f();

            if result.is_ok() | self.stop.should_stop(attempt, started.elapsed()) {
                return (
                    result,
                    RetryInfo {
                        attempts: attempt,
                        elapsed: started.elapsed(),
                    },
                );
            }

            sleep(self.wait.wait_duration(attempt));
            attempt += 1;
        }
    }

    pub fn call<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        let (res, _) = self.call_with_info(f);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        strategies::{stop::StopAfterAttempts, wait::WaitFixed},
        Retry,
    };

    #[test]
    fn presets() {
        let _basic_retry = Retry::basic();
        let _exponential_retry = Retry::exponential();
    }

    #[test]
    fn call_with_info() {
        let mut it = 0..3;

        let (result, retry_info) = Retry::basic()
            .stop(StopAfterAttempts::from(3u16))
            .wait(WaitFixed::from_millis(100))
            .call_with_info(|| -> Result<(), ()> {
                if it.next().is_some() {
                    return Err(());
                } else {
                    return Ok(());
                }
            });

        assert!(result.is_ok());
        assert_eq!(retry_info.attempts, 3)
    }

    #[test]
    fn call() {
        let mut it = 0..3;

        let result = Retry::basic()
            .stop(StopAfterAttempts::from(3u16))
            .wait(WaitFixed::from_millis(100))
            .call(|| -> Result<(), ()> {
                if it.next().is_some() {
                    return Err(());
                } else {
                    return Ok(());
                }
            });

        assert!(result.is_ok());
    }
}
