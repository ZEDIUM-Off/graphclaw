use crate::MemgraphError;
use serde::{Deserialize, Serialize};
use std::env;

/// Connection configuration for Memgraph (Bolt).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemgraphConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

impl Default for MemgraphConfig {
    fn default() -> Self {
        Self {
            host: env_var("MEMGRAPH_HOST").unwrap_or_else(|| "127.0.0.1".to_string()),
            port: env_var("MEMGRAPH_BOLT_PORT")
                .or_else(|| env_var("MEMGRAPH_PORT"))
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(7687),
            user: env_var("MEMGRAPH_USER").unwrap_or_else(|| "admin".to_string()),
            password: env_var("MEMGRAPH_PASSWORD").unwrap_or_else(|| "adminpassword".to_string()),
        }
    }
}

fn env_var(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

impl MemgraphConfig {
    /// Bolt URI for the driver (e.g. `127.0.0.1:7687`).
    pub fn bolt_uri(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Validate configuration.
    pub fn validate(&self) -> Result<(), MemgraphError> {
        if self.host.is_empty() {
            return Err(MemgraphError::Config("host cannot be empty".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MemgraphConfig;

    #[test]
    fn default_uses_local_playground_fallbacks() {
        unsafe {
            std::env::remove_var("MEMGRAPH_HOST");
            std::env::remove_var("MEMGRAPH_BOLT_PORT");
            std::env::remove_var("MEMGRAPH_PORT");
            std::env::remove_var("MEMGRAPH_USER");
            std::env::remove_var("MEMGRAPH_PASSWORD");
        }

        let config = MemgraphConfig::default();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 7687);
        assert_eq!(config.user, "admin");
        assert_eq!(config.password, "adminpassword");
    }

    #[test]
    fn default_prefers_memgraph_environment_over_fallbacks() {
        unsafe {
            std::env::set_var("MEMGRAPH_HOST", "memgraph.internal");
            std::env::set_var("MEMGRAPH_BOLT_PORT", "9999");
            std::env::set_var("MEMGRAPH_USER", "playground");
            std::env::set_var("MEMGRAPH_PASSWORD", "secret");
        }

        let config = MemgraphConfig::default();

        assert_eq!(config.host, "memgraph.internal");
        assert_eq!(config.port, 9999);
        assert_eq!(config.user, "playground");
        assert_eq!(config.password, "secret");

        unsafe {
            std::env::remove_var("MEMGRAPH_HOST");
            std::env::remove_var("MEMGRAPH_BOLT_PORT");
            std::env::remove_var("MEMGRAPH_USER");
            std::env::remove_var("MEMGRAPH_PASSWORD");
        }
    }
}
