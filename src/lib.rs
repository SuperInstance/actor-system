//! actor-system — core actor runtime and system configuration.

use std::sync::Arc;

/// Configuration for the actor system.
#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub name: String,
    pub worker_threads: usize,
    pub mailbox_capacity: usize,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            name: "default".into(),
            worker_threads: 4,
            mailbox_capacity: 1024,
        }
    }
}

/// Handle to a running actor system.
#[derive(Debug, Clone)]
pub struct ActorSystem {
    config: Arc<SystemConfig>,
}

impl ActorSystem {
    pub fn new(config: SystemConfig) -> Self {
        Self { config: Arc::new(config) }
    }

    pub fn config(&self) -> &SystemConfig {
        &self.config
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn shutdown(&self) {
        // placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_default_config() {
        let sys = ActorSystem::new(SystemConfig::default());
        assert_eq!(sys.name(), "default");
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
