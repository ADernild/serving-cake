use dashmap::DashMap;
use std::net::IpAddr;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: DashMap<IpAddr, Instant>,
    limit: Duration,
}

impl RateLimiter {
    pub fn new(limit_secs: u64) -> Self {
        Self {
            requests: DashMap::new(),
            limit: Duration::from_secs(limit_secs),
        }
    }

    pub fn check(&self, ip: IpAddr) -> Result<(), String> {
        if let Some(mut entry) = self.requests.get_mut(&ip) {
            if entry.value().elapsed() < self.limit {
                return Err("Greedy! Wait a minute before asking for more cake! 🍰⏳".to_string());
            }
            *entry.value_mut() = Instant::now();
        } else {
            self.requests.insert(ip, Instant::now());
        }
        Ok(())
    }
}
