# Stubborn

_I didn't like the other retry crates so here is another one!_

------------------------------------------------------------------------

**Stubborn** is a flexible and composable retry library for Rust,
inspired by Python's tenacity.

It gives you explicit control over:

-   When to stop retrying
-   How long to wait between attempts
-   Access to retry metadata (attempt count, elapsed time)

------------------------------------------------------------------------

## Features

-   Pluggable stop strategies
-   Pluggable wait strategies
-   Presets for common retry patterns
-   Access to retry metadata via `RetryInfo`
-   Minimal and dependency-light design

------------------------------------------------------------------------

## Installation

``` toml
[dependencies]
stubborn = "0.1"
```

------------------------------------------------------------------------

## Quick Start

### Basic retry (3 attempts, 1s fixed delay)

``` rust
use stubborn::Retry;

let result = Retry::basic().call(|| {
    do_something_that_might_fail()
});
```

------------------------------------------------------------------------

### Exponential backoff

``` rust
use stubborn::Retry;

let result = Retry::exponential().call(|| {
    do_something_that_might_fail()
});
```

Default behavior:

-   3 attempts
-   Starts at 1 second
-   Multiplier: 2.0

------------------------------------------------------------------------

## Custom Configuration

Override stop and wait strategies:

``` rust
use stubborn::{
    Retry,
    strategies::{
        stop::StopAfterAttempts,
        wait::WaitFixed,
    },
};

let result = Retry::basic()
    .stop(StopAfterAttempts::from(5u16))
    .wait(WaitFixed::from_millis(200))
    .call(|| -> Result<(), ()> {
        might_fail()
    });
```

------------------------------------------------------------------------

## Access Retry Metadata

Use `call_with_info` to retrieve retry statistics:

``` rust
let (result, info) = Retry::basic().call_with_info(|| {
    might_fail()
});

println!("Attempts: {}", info.attempts);
println!("Elapsed: {:?}", info.elapsed);
```

Example structure:

``` rust
pub struct RetryInfo {
    pub attempts: usize,
    pub elapsed: Duration,
}
```

------------------------------------------------------------------------

## Current Strategies

### Stop Strategies

-   `StopAfterAttempts`

### Wait Strategies

-   `WaitFixed`
-   `WaitExponential`

------------------------------------------------------------------------

## Limitations

-   Blocking (uses `std::thread::sleep`)
-   No async support yet
-   No jitter strategy (yet)
-   No predicate-based retry filtering (yet)

------------------------------------------------------------------------

## Roadmap Ideas

-   Jittered exponential backoff
-   Stop after elapsed duration
-   Retry based on error predicates
-   Extension trait for ergonomic `.retry()` usage
-   Async support (tokio / async-std)


------------------------------------------------------------------------
