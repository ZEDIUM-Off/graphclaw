use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemgraphError {
    #[error("connection failed: {0}")]
    Connection(String),

    #[error("query execution failed: {0}")]
    Query(String),

    #[error("no result returned where one was expected")]
    NoResult,

    #[error("invalid node id")]
    InvalidNodeId,

    #[error("invalid configuration: {0}")]
    Config(String),
}
