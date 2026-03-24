//! Low-level Memgraph (Bolt) client for GraphClaw.
//!
//! This crate provides graph primitives only: connection, create node/edge,
//! read subgraph, neighborhood. It does not contain View logic; that lives in
//! the `sets` crate.

mod client;
mod config;
mod error;
mod types;

pub use client::MemgraphClient;
pub use config::MemgraphConfig;
pub use error::MemgraphError;
pub use types::{GraphEdge, GraphNode, GraphSnapshot, Subgraph};
