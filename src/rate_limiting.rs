use chrono::{DateTime, Utc};
use reqwest::Response;
use tokio::time::{delay_for, Delay};

#[derive(Debug)]
pub(crate) struct RateLimiter {
    /// The state of the `RateLimiter`.
    pub state: RateLimiterState,
    /// The number of requests currently running.
    pub current_count: u32,
}

#[derive(Debug)]
pub(crate) enum RateLimiterState {
    /// An undefined state, where we don't know anthing
    /// about the rates for the client, e.g. at startup.
    Undefined,
    /// A state where we can estimate the number of remaining requests
    /// based on past requests, and the end of the current time window.
    Estimated(u32, DateTime<Utc>),
    /// A state where we know we are being rate-limited until the given time.
    RateLimited(DateTime<Utc>),
}

impl RateLimiter {
    /// Creates a new blank `RateLimiter`
    pub fn new() -> Self {
        Self {
            state: RateLimiterState::Undefined,
            current_count: 0,
        }
    }

    pub fn request_delay(&mut self) -> Option<Delay> {
        match &self.state {
            RateLimiterState::Undefined => None,
            RateLimiterState::Estimated(remaining, reset) => {
                let now = Utc::now();
                if now > *reset {
                    self.state = RateLimiterState::Undefined;
                    None
                } else if remaining - self.current_count > 0 {
                    None
                } else {
                    Some(*reset - now)
                }
            }
            RateLimiterState::RateLimited(reset) => {
                let now = Utc::now();
                if now > *reset {
                    self.state = RateLimiterState::Undefined;
                    None
                } else {
                    Some(*reset - Utc::now())
                }
            }
        }
        .map(|d| delay_for(d.to_std().unwrap()))
    }

    pub fn register_request(&mut self) {
        self.current_count += 1;
    }

    pub fn register_response(&mut self, res: &crate::Result<Response>) {
        if let Ok(ref res) = res {
            let headers = res.headers();
            let remaining = headers.get("X-RateLimit-Remaining")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok());
            let reset = headers.get("X-RateLimit-Reset")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok());

            if let (Some(remaining), Some(reset)) = (remaining, reset) {
                if remaining > 0 {
                    self.state = RateLimiterState::Estimated(remaining, reset);
                } else {
                    self.state = RateLimiterState::RateLimited(reset);
                }
            }
        }
        self.current_count -= 1;
    }
}
