use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetsError {
    #[error("set not found: {0}")]
    SetNotFound(String),

    #[error("set composition cycle detected at: {0}")]
    SetCycle(String),

    #[error("memgraph error: {0}")]
    Memgraph(#[from] memgraph::MemgraphError),

    #[error("invalid operation: {0}")]
    InvalidOperation(String),

    #[error("invalid predicate: {0}")]
    InvalidPredicate(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::SetsError;

    #[test]
    fn error_messages_are_stable_and_specific() {
        let not_found = SetsError::SetNotFound("set_x".to_string());
        let cycle = SetsError::SetCycle("set_cycle".to_string());
        let invalid_predicate = SetsError::InvalidPredicate("bad".to_string());

        assert_eq!(not_found.to_string(), "set not found: set_x");
        assert_eq!(
            cycle.to_string(),
            "set composition cycle detected at: set_cycle"
        );
        assert_eq!(invalid_predicate.to_string(), "invalid predicate: bad");
    }

    #[test]
    fn memgraph_errors_convert_into_sets_errors() {
        let source = memgraph::MemgraphError::InvalidNodeId;
        let error: SetsError = source.into();

        assert_eq!(error.to_string(), "memgraph error: invalid node id");
    }
}
