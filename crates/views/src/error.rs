use thiserror::Error;

#[derive(Error, Debug)]
pub enum ViewsError {
    #[error("template not found: {0}")]
    TemplateNotFound(String),

    #[error("template composition cycle detected at: {0}")]
    TemplateCycle(String),

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
    use super::ViewsError;

    #[test]
    fn error_messages_are_stable_and_specific() {
        let not_found = ViewsError::TemplateNotFound("view_x".to_string());
        let cycle = ViewsError::TemplateCycle("view_cycle".to_string());
        let invalid_predicate = ViewsError::InvalidPredicate("bad".to_string());

        assert_eq!(not_found.to_string(), "template not found: view_x");
        assert_eq!(
            cycle.to_string(),
            "template composition cycle detected at: view_cycle"
        );
        assert_eq!(invalid_predicate.to_string(), "invalid predicate: bad");
    }

    #[test]
    fn memgraph_errors_convert_into_views_errors() {
        let source = memgraph::MemgraphError::InvalidNodeId;
        let error: ViewsError = source.into();

        assert_eq!(error.to_string(), "memgraph error: invalid node id");
    }
}
