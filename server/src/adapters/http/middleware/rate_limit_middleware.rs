use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DashMapStateStore};
use std::{num::NonZeroU32, sync::Arc};
use uuid::Uuid;

/// Rate limiter for user-based request limiting
pub struct UserRateLimiter {
    limiter: Arc<RateLimiter<Uuid, DashMapStateStore<Uuid>, DefaultClock>>,
}

impl UserRateLimiter {
    /// Create a new user rate limiter
    ///
    /// # Arguments
    /// * `requests_per_second` - Maximum requests per second per user
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(requests_per_second).unwrap());
        let limiter = Arc::new(RateLimiter::keyed(quota));
        Self { limiter }
    }

    /// Check if a user can make a request
    ///
    /// Returns true if the request is allowed, false if rate limit is exceeded
    pub fn check(&self, user_id: Uuid) -> bool {
        self.limiter.check_key(&user_id).is_ok()
    }
}

impl Clone for UserRateLimiter {
    fn clone(&self) -> Self {
        Self {
            limiter: Arc::clone(&self.limiter),
        }
    }
}
