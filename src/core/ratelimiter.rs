pub use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};

pub struct IndexRateLimiter;
pub struct RegisterRateLimiter;
pub struct VerifyRateLimiter;
pub struct UnregisterRateLimiter;
pub struct LoginRateLimiter;
pub struct GenericRateLimiter;
pub struct ActionRateLimiter;
pub struct QueueRateLimiter;
pub struct ContactRateLimiter;

/* Rate limiters traits */

impl<'a> RocketGovernable<'a> for IndexRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(10u32))
    }
}

impl<'a> RocketGovernable<'a> for RegisterRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(30u32))
    }
}

impl<'a> RocketGovernable<'a> for VerifyRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(30u32))
    }
}

impl<'a> RocketGovernable<'a> for UnregisterRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(50u32))
    }
}

impl<'a> RocketGovernable<'a> for LoginRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(30u32))
    }
}

impl<'a> RocketGovernable<'a> for ActionRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(50u32))
    }
}

impl<'a> RocketGovernable<'a> for QueueRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(50u32))
    }
}

impl<'a> RocketGovernable<'a> for ContactRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(50u32))
    }
}

impl<'a> RocketGovernable<'a> for GenericRateLimiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(1u32))
    }
}
