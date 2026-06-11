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
