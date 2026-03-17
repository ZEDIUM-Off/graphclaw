use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A graph node (vertex) with optional labels and properties.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphNode {
    /// Internal graph ID (e.g. Memgraph/Neo4j node id).
    pub id: i64,
    /// Node labels (e.g. `["Concept", "Business"]`).
    pub labels: Vec<String>,
    /// Properties as JSON-friendly values.
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// A graph edge (relationship) between two nodes.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphEdge {
    /// Internal relationship ID.
    pub id: i64,
    /// Source node internal id.
    pub from_id: i64,
    /// Target node internal id.
    pub to_id: i64,
    /// Relationship type (e.g. `RELATES_TO`).
    pub rel_type: String,
    /// Properties as JSON-friendly values.
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// A subgraph: set of nodes and edges.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Subgraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// A bounded graph snapshot for operator-facing exploration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub subgraph: Subgraph,
    pub truncated: bool,
    pub node_limit: u32,
    pub edge_limit: u32,
}
