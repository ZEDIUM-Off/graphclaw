//! Memgraph client using the official `rsmgclient` driver.

use crate::config::MemgraphConfig;
use crate::error::MemgraphError;
use crate::types::{GraphEdge, GraphNode, GraphSnapshot, Subgraph};
use rsmgclient::{
    ConnectParams, Connection, ConnectionStatus, Node, QueryParam, Record, Relationship, SSLMode,
    Value,
};
use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, Once};

static MGCLIENT_INIT: Once = Once::new();

type ConnectionTask = Box<dyn FnOnce(&mut Connection) + Send + 'static>;

/// Low-level Memgraph client.
/// Handles connection and graph primitives only.
pub struct MemgraphClient {
    task_tx: mpsc::Sender<ConnectionTask>,
}

impl MemgraphClient {
    /// Connect using the given config.
    pub async fn connect(config: &MemgraphConfig) -> Result<Self, MemgraphError> {
        config.validate()?;
        let config = config.clone();
        let (task_tx, task_rx) = mpsc::channel::<ConnectionTask>();
        let (ready_tx, ready_rx) = mpsc::channel::<Result<(), MemgraphError>>();

        std::thread::Builder::new()
            .name("graphclaw-memgraph".to_string())
            .spawn(move || {
                MGCLIENT_INIT.call_once(Connection::init);
                let params = build_connect_params(&config);

                let connection = Connection::connect(&params)
                    .map_err(|e| MemgraphError::Connection(e.to_string()))
                    .and_then(|connection| {
                        if connection.status() == ConnectionStatus::Ready {
                            Ok(connection)
                        } else {
                            Err(MemgraphError::Connection(format!(
                                "unexpected connection status: {:?}",
                                connection.status()
                            )))
                        }
                    });

                let mut connection = match connection {
                    Ok(connection) => {
                        let _ = ready_tx.send(Ok(()));
                        connection
                    }
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };

                for task in task_rx {
                    task(&mut connection);
                }
                connection.close();
            })
            .map_err(|error| MemgraphError::Connection(error.to_string()))?;

        ready_rx
            .recv()
            .map_err(|error| MemgraphError::Connection(error.to_string()))??;

        Ok(Self { task_tx })
    }

    /// Execute a Cypher query without returning rows.
    pub async fn run(&self, cypher: &str) -> Result<(), MemgraphError> {
        let cypher = cypher.to_string();
        self.with_connection(move |connection| {
            connection
                .execute_without_results(&cypher)
                .map_err(|e| MemgraphError::Query(e.to_string()))?;
            connection
                .commit()
                .map_err(|e| MemgraphError::Query(e.to_string()))?;
            Ok(())
        })
        .await
    }

    /// Create a node with the given labels and properties. Returns the created node.
    pub async fn create_node(
        &self,
        labels: &[String],
        properties: &HashMap<String, serde_json::Value>,
    ) -> Result<GraphNode, MemgraphError> {
        let labels_part = build_labels(labels)?;
        let mut params = HashMap::new();
        params.insert(
            "props".to_string(),
            json_value_to_query_param_map(properties),
        );

        let query = format!("CREATE (n{labels_part}) SET n += $props RETURN n");
        self.with_connection(move |connection| {
            let records = execute_records(connection, &query, Some(&params))?;
            first_node(&records)
        })
        .await
    }

    /// Create a relationship between two nodes by internal id. Returns the created edge.
    pub async fn create_edge(
        &self,
        from_id: i64,
        to_id: i64,
        rel_type: &str,
        properties: &HashMap<String, serde_json::Value>,
    ) -> Result<GraphEdge, MemgraphError> {
        let rel_type = sanitize_identifier(rel_type, "relationship type")?;
        let mut params = HashMap::new();
        params.insert("from_id".to_string(), QueryParam::Int(from_id));
        params.insert("to_id".to_string(), QueryParam::Int(to_id));
        params.insert(
            "props".to_string(),
            json_value_to_query_param_map(properties),
        );

        let query = format!(
            "MATCH (a), (b) \
             WHERE id(a) = $from_id AND id(b) = $to_id \
             CREATE (a)-[r:{rel_type}]->(b) \
             SET r += $props \
             RETURN r"
        );
        self.with_connection(move |connection| {
            let records = execute_records(connection, &query, Some(&params))?;
            first_edge(&records)
        })
        .await
    }

    /// Return a subgraph containing the given node ids and all edges between them.
    pub async fn get_subgraph(&self, node_ids: &[i64]) -> Result<Subgraph, MemgraphError> {
        if node_ids.is_empty() {
            return Ok(Subgraph::default());
        }

        let params = node_ids_param(node_ids);
        self.with_connection(move |connection| {
            let nodes = execute_records(
                connection,
                "MATCH (n) WHERE id(n) IN $node_ids RETURN n",
                Some(&params),
            )?;
            let edges = execute_records(
                connection,
                "MATCH (a)-[r]-(b) \
                 WHERE id(a) IN $node_ids AND id(b) IN $node_ids \
                 RETURN DISTINCT r",
                Some(&params),
            )?;

            let nodes = nodes
                .into_iter()
                .flat_map(|record| record.values.into_iter())
                .filter_map(value_to_node)
                .collect::<Vec<_>>();
            let edges = edges
                .into_iter()
                .flat_map(|record| record.values.into_iter())
                .filter_map(value_to_edge)
                .collect::<Vec<_>>();

            Ok(normalize_subgraph(Subgraph { nodes, edges }))
        })
        .await
    }

    /// Return a bounded graph snapshot suitable for default playground rendering.
    pub async fn get_graph_snapshot(
        &self,
        node_limit: u32,
        edge_limit: u32,
    ) -> Result<GraphSnapshot, MemgraphError> {
        let node_limit = node_limit.clamp(1, 10_000);
        let edge_limit = edge_limit.clamp(1, 20_000);
        let node_fetch_limit = i64::from(node_limit.saturating_add(1));
        let edge_fetch_limit = i64::from(edge_limit.saturating_add(1));

        self.with_connection(move |connection| {
            let mut node_limit_params = HashMap::new();
            node_limit_params.insert("node_limit".to_string(), QueryParam::Int(node_fetch_limit));

            let node_records = execute_records(
                connection,
                "MATCH (n) RETURN n ORDER BY id(n) LIMIT $node_limit",
                Some(&node_limit_params),
            )?;
            let mut nodes = node_records
                .into_iter()
                .flat_map(|record| record.values.into_iter())
                .filter_map(value_to_node)
                .collect::<Vec<_>>();
            let nodes_truncated = nodes.len() > node_limit as usize;
            if nodes_truncated {
                nodes.truncate(node_limit as usize);
            }

            if nodes.is_empty() {
                return Ok(GraphSnapshot {
                    subgraph: Subgraph::default(),
                    truncated: false,
                    node_limit,
                    edge_limit,
                });
            }

            let node_ids = nodes.iter().map(|node| node.id).collect::<Vec<_>>();
            let mut edge_params = node_ids_param(&node_ids);
            edge_params.insert("edge_limit".to_string(), QueryParam::Int(edge_fetch_limit));
            let edge_records = execute_records(
                connection,
                "MATCH (a)-[r]-(b) \
                 WHERE id(a) IN $node_ids AND id(b) IN $node_ids \
                 RETURN DISTINCT r ORDER BY id(r) LIMIT $edge_limit",
                Some(&edge_params),
            )?;
            let mut edges = edge_records
                .into_iter()
                .flat_map(|record| record.values.into_iter())
                .filter_map(value_to_edge)
                .collect::<Vec<_>>();
            let edges_truncated = edges.len() > edge_limit as usize;
            if edges_truncated {
                edges.truncate(edge_limit as usize);
            }

            Ok(GraphSnapshot {
                subgraph: normalize_subgraph(Subgraph { nodes, edges }),
                truncated: nodes_truncated || edges_truncated,
                node_limit,
                edge_limit,
            })
        })
        .await
    }

    /// Select nodes by optional label and property filters.
    pub async fn select_nodes(
        &self,
        label: Option<&str>,
        props: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<GraphNode>, MemgraphError> {
        if label.is_none() && props.is_empty() {
            return Ok(Vec::new());
        }

        let label_part = if let Some(label) = label {
            format!(":{}", sanitize_identifier(label, "label")?)
        } else {
            String::new()
        };

        let mut params = HashMap::new();
        let mut predicates = Vec::new();
        for (key, value) in props {
            let key = sanitize_identifier(key, "property key")?;
            let param_name = format!("prop_{key}");
            params.insert(param_name.clone(), json_value_to_query_param(value));
            predicates.push(format!("n.{key} = ${param_name}"));
        }

        let where_clause = if predicates.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", predicates.join(" AND "))
        };
        let query = format!("MATCH (n{label_part}){where_clause} RETURN n");

        self.with_connection(move |connection| {
            let records = execute_records(connection, &query, Some(&params))?;
            let nodes = records
                .into_iter()
                .flat_map(|record| record.values.into_iter())
                .filter_map(value_to_node)
                .collect::<Vec<_>>();
            Ok(sort_nodes(dedup_nodes(nodes)))
        })
        .await
    }

    /// Expand from one or more seeds: return all nodes and edges on matching paths.
    pub async fn neighborhood(
        &self,
        seed_ids: &[i64],
        relation_type: Option<&str>,
        depth: u32,
    ) -> Result<Subgraph, MemgraphError> {
        if seed_ids.is_empty() {
            return Ok(Subgraph::default());
        }

        let depth = depth.clamp(1, 10);
        let rel_filter = relation_type
            .map(|relation| sanitize_identifier(relation, "relationship type"))
            .transpose()?
            .map(|relation| format!(":{relation}"))
            .unwrap_or_default();

        let query = format!(
            "UNWIND $seed_ids AS seed_id \
             MATCH path = (start)-[r{rel_filter}*1..{depth}]-(end) \
             WHERE id(start) = seed_id \
             RETURN nodes(path), relationships(path)"
        );
        let params = node_ids_param(seed_ids);

        self.with_connection(move |connection| {
            let records = execute_records(connection, &query, Some(&params))?;
            let mut nodes = Vec::new();
            let mut edges = Vec::new();

            for record in records {
                if let Some(path_nodes) = record.values.first().and_then(value_to_node_list) {
                    nodes.extend(path_nodes);
                }
                if let Some(path_edges) = record.values.get(1).and_then(value_to_edge_list) {
                    edges.extend(path_edges);
                }
            }

            Ok(normalize_subgraph(Subgraph { nodes, edges }))
        })
        .await
    }

    /// Inspect a single node by id. Returns None if not found.
    pub async fn inspect_node(&self, node_id: i64) -> Result<Option<GraphNode>, MemgraphError> {
        let mut params = HashMap::new();
        params.insert("node_id".to_string(), QueryParam::Int(node_id));

        self.with_connection(move |connection| {
            let records = execute_records(
                connection,
                "MATCH (n) WHERE id(n) = $node_id RETURN n",
                Some(&params),
            )?;
            Ok(records
                .into_iter()
                .flat_map(|record| record.values.into_iter())
                .find_map(value_to_node))
        })
        .await
    }

    async fn with_connection<T, F>(&self, operation: F) -> Result<T, MemgraphError>
    where
        T: Send + 'static,
        F: FnOnce(&mut Connection) -> Result<T, MemgraphError> + Send + 'static,
    {
        let task_tx = self.task_tx.clone();
        tokio::task::spawn_blocking(move || {
            let (reply_tx, reply_rx) = mpsc::channel::<Result<T, MemgraphError>>();
            task_tx
                .send(Box::new(move |connection| {
                    let _ = reply_tx.send(operation(connection));
                }))
                .map_err(|error| MemgraphError::Connection(error.to_string()))?;

            reply_rx
                .recv()
                .map_err(|error| MemgraphError::Connection(error.to_string()))?
        })
        .await
        .map_err(|error| MemgraphError::Connection(error.to_string()))?
    }
}

fn build_connect_params(config: &MemgraphConfig) -> ConnectParams {
    ConnectParams {
        host: Some(config.host.clone()),
        port: config.port,
        username: if config.user.is_empty() {
            None
        } else {
            Some(config.user.clone())
        },
        password: if config.password.is_empty() {
            None
        } else {
            Some(config.password.clone())
        },
        sslmode: SSLMode::Disable,
        lazy: false,
        autocommit: true,
        ..Default::default()
    }
}

fn execute_records(
    connection: &mut Connection,
    query: &str,
    params: Option<&HashMap<String, QueryParam>>,
) -> Result<Vec<Record>, MemgraphError> {
    connection
        .execute(query, params)
        .map_err(|e| MemgraphError::Query(e.to_string()))?;
    connection
        .fetchall()
        .map_err(|e| MemgraphError::Query(e.to_string()))
}

fn first_node(records: &[Record]) -> Result<GraphNode, MemgraphError> {
    records
        .iter()
        .flat_map(|record| record.values.iter())
        .find_map(|value| value_to_node(value.clone()))
        .ok_or(MemgraphError::NoResult)
}

fn first_edge(records: &[Record]) -> Result<GraphEdge, MemgraphError> {
    records
        .iter()
        .flat_map(|record| record.values.iter())
        .find_map(|value| value_to_edge(value.clone()))
        .ok_or(MemgraphError::NoResult)
}

fn node_ids_param(node_ids: &[i64]) -> HashMap<String, QueryParam> {
    let mut params = HashMap::new();
    params.insert(
        "node_ids".to_string(),
        QueryParam::List(node_ids.iter().copied().map(QueryParam::Int).collect()),
    );
    params.insert(
        "seed_ids".to_string(),
        QueryParam::List(node_ids.iter().copied().map(QueryParam::Int).collect()),
    );
    params
}

fn build_labels(labels: &[String]) -> Result<String, MemgraphError> {
    let mut parts = Vec::new();
    for label in labels {
        parts.push(format!(":{}", sanitize_identifier(label, "label")?));
    }
    Ok(parts.join(""))
}

fn sanitize_identifier(identifier: &str, kind: &str) -> Result<String, MemgraphError> {
    if identifier.is_empty() {
        return Err(MemgraphError::Query(format!("{kind} cannot be empty")));
    }

    let mut chars = identifier.chars();
    let Some(first) = chars.next() else {
        return Err(MemgraphError::Query(format!("{kind} cannot be empty")));
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(MemgraphError::Query(format!(
            "{kind} must start with a letter or underscore"
        )));
    }
    if !chars.all(|char| char.is_ascii_alphanumeric() || char == '_') {
        return Err(MemgraphError::Query(format!(
            "{kind} must contain only ASCII alphanumeric characters or underscores"
        )));
    }

    Ok(identifier.to_string())
}

fn value_to_node(value: Value) -> Option<GraphNode> {
    match value {
        Value::Node(node) => Some(node_to_graph_node(node)),
        _ => None,
    }
}

fn value_to_edge(value: Value) -> Option<GraphEdge> {
    match value {
        Value::Relationship(edge) => Some(relationship_to_graph_edge(edge)),
        _ => None,
    }
}

fn value_to_node_list(value: &Value) -> Option<Vec<GraphNode>> {
    match value {
        Value::List(values) => Some(
            values
                .iter()
                .filter_map(|value| value_to_node(value.clone()))
                .collect(),
        ),
        _ => None,
    }
}

fn value_to_edge_list(value: &Value) -> Option<Vec<GraphEdge>> {
    match value {
        Value::List(values) => Some(
            values
                .iter()
                .filter_map(|value| value_to_edge(value.clone()))
                .collect(),
        ),
        _ => None,
    }
}

fn node_to_graph_node(node: Node) -> GraphNode {
    GraphNode {
        id: node.id,
        labels: node.labels,
        properties: value_map_to_json_map(node.properties),
    }
}

fn relationship_to_graph_edge(relationship: Relationship) -> GraphEdge {
    GraphEdge {
        id: relationship.id,
        from_id: relationship.start_id,
        to_id: relationship.end_id,
        rel_type: relationship.type_,
        properties: value_map_to_json_map(relationship.properties),
    }
}

fn value_map_to_json_map(values: HashMap<String, Value>) -> HashMap<String, serde_json::Value> {
    values
        .into_iter()
        .map(|(key, value)| (key, value_to_json(value)))
        .collect()
}

fn value_to_json(value: Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(value) => serde_json::Value::Bool(value),
        Value::Int(value) => serde_json::Value::Number(value.into()),
        Value::Float(value) => serde_json::Number::from_f64(value)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::String(value) => serde_json::Value::String(value),
        Value::List(values) => {
            serde_json::Value::Array(values.into_iter().map(value_to_json).collect())
        }
        Value::Map(values) => serde_json::Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, value_to_json(value)))
                .collect(),
        ),
        Value::Date(value) => serde_json::Value::String(value.to_string()),
        Value::LocalTime(value) => serde_json::Value::String(value.to_string()),
        Value::LocalDateTime(value) => serde_json::Value::String(value.to_string()),
        Value::DateTime(value) => serde_json::Value::String(format!("{value:?}")),
        Value::Duration(value) => serde_json::Value::String(value.to_string()),
        Value::Point2D(value) => serde_json::Value::String(value.to_string()),
        Value::Point3D(value) => serde_json::Value::String(value.to_string()),
        Value::Node(node) => serde_json::json!({
            "id": node.id,
            "labels": node.labels,
            "properties": value_map_to_json_map(node.properties),
        }),
        Value::Relationship(relationship) => serde_json::json!({
            "id": relationship.id,
            "from_id": relationship.start_id,
            "to_id": relationship.end_id,
            "rel_type": relationship.type_,
            "properties": value_map_to_json_map(relationship.properties),
        }),
        Value::UnboundRelationship(relationship) => serde_json::json!({
            "id": relationship.id,
            "rel_type": relationship.type_,
            "properties": value_map_to_json_map(relationship.properties),
        }),
        Value::Path(path) => serde_json::json!({
            "node_count": path.node_count,
            "relationship_count": path.relationship_count,
        }),
    }
}

fn json_value_to_query_param_map(properties: &HashMap<String, serde_json::Value>) -> QueryParam {
    QueryParam::Map(
        properties
            .iter()
            .map(|(key, value)| (key.clone(), json_value_to_query_param(value)))
            .collect(),
    )
}

fn json_value_to_query_param(value: &serde_json::Value) -> QueryParam {
    match value {
        serde_json::Value::Null => QueryParam::Null,
        serde_json::Value::Bool(value) => QueryParam::Bool(*value),
        serde_json::Value::Number(value) => {
            if let Some(int) = value.as_i64() {
                QueryParam::Int(int)
            } else if let Some(float) = value.as_f64() {
                QueryParam::Float(float)
            } else {
                QueryParam::Null
            }
        }
        serde_json::Value::String(value) => QueryParam::String(value.clone()),
        serde_json::Value::Array(values) => {
            QueryParam::List(values.iter().map(json_value_to_query_param).collect())
        }
        serde_json::Value::Object(values) => QueryParam::Map(
            values
                .iter()
                .map(|(key, value)| (key.clone(), json_value_to_query_param(value)))
                .collect(),
        ),
    }
}

fn normalize_subgraph(subgraph: Subgraph) -> Subgraph {
    let nodes = sort_nodes(dedup_nodes(subgraph.nodes));
    let node_ids = nodes.iter().map(|node| node.id).collect::<HashSet<_>>();
    let edges = sort_edges(dedup_edges(
        subgraph
            .edges
            .into_iter()
            .filter(|edge| node_ids.contains(&edge.from_id) && node_ids.contains(&edge.to_id))
            .collect(),
    ));

    Subgraph { nodes, edges }
}

fn dedup_nodes(nodes: Vec<GraphNode>) -> Vec<GraphNode> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for node in nodes {
        if seen.insert(node.id) {
            deduped.push(node);
        }
    }
    deduped
}

fn dedup_edges(edges: Vec<GraphEdge>) -> Vec<GraphEdge> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for edge in edges {
        if seen.insert(edge.id) {
            deduped.push(edge);
        }
    }
    deduped
}

fn sort_nodes(mut nodes: Vec<GraphNode>) -> Vec<GraphNode> {
    nodes.sort_by_key(|node| node.id);
    nodes
}

fn sort_edges(mut edges: Vec<GraphEdge>) -> Vec<GraphEdge> {
    edges.sort_by_key(|edge| edge.id);
    edges
}

#[cfg(test)]
mod tests {
    use super::{normalize_subgraph, sanitize_identifier};
    use crate::{GraphEdge, GraphNode, GraphSnapshot, Subgraph};
    use std::collections::HashMap;

    #[test]
    fn normalize_subgraph_drops_open_edges_and_sorts_results() {
        let subgraph = Subgraph {
            nodes: vec![
                GraphNode {
                    id: 2,
                    labels: vec!["Concept".to_string()],
                    properties: HashMap::new(),
                },
                GraphNode {
                    id: 1,
                    labels: vec!["Concept".to_string()],
                    properties: HashMap::new(),
                },
            ],
            edges: vec![
                GraphEdge {
                    id: 2,
                    from_id: 1,
                    to_id: 2,
                    rel_type: "RELATES_TO".to_string(),
                    properties: HashMap::new(),
                },
                GraphEdge {
                    id: 1,
                    from_id: 1,
                    to_id: 9,
                    rel_type: "LEAKS_TO".to_string(),
                    properties: HashMap::new(),
                },
            ],
        };

        let normalized = normalize_subgraph(subgraph);
        assert_eq!(
            normalized
                .nodes
                .iter()
                .map(|node| node.id)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            normalized
                .edges
                .iter()
                .map(|edge| edge.id)
                .collect::<Vec<_>>(),
            vec![2]
        );
    }

    #[test]
    fn sanitize_identifier_rejects_cypher_injection_like_values() {
        assert!(sanitize_identifier("Concept", "label").is_ok());
        assert!(sanitize_identifier("Concept-Name", "label").is_err());
        assert!(sanitize_identifier("RELATES_TO RETURN 1", "relationship type").is_err());
    }

    #[test]
    fn graph_snapshot_defaults_are_untruncated_and_empty() {
        let snapshot = GraphSnapshot::default();

        assert!(snapshot.subgraph.nodes.is_empty());
        assert!(snapshot.subgraph.edges.is_empty());
        assert!(!snapshot.truncated);
        assert_eq!(snapshot.node_limit, 0);
        assert_eq!(snapshot.edge_limit, 0);
    }
}
