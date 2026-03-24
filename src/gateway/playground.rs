//! Playground API: graph primitives and set composition/resolution/export.
//! Routes under /api/playground/*. DTOs only; no leakage of internal types.

use super::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use memgraph::{GraphSnapshot, MemgraphClient, MemgraphConfig};
use serde::{Deserialize, Serialize};
use sets::{export_for_llm, BoundSet, ResolvedSet, SetDefinition, SetSelectors, SetsService};
use std::collections::HashMap;
use std::sync::Arc;

const DEFAULT_GRAPH_NODE_LIMIT: u32 = 200;
const DEFAULT_GRAPH_EDGE_LIMIT: u32 = 400;
const MAX_GRAPH_NODE_LIMIT: u32 = 1_000;
const MAX_GRAPH_EDGE_LIMIT: u32 = 5_000;
const ROOT_SET_ID: &str = "root";

/// Playground state (sets service; holds Memgraph client internally).
#[derive(Clone)]
pub struct PlaygroundState {
    pub sets: Arc<SetsService>,
    pub memgraph: Arc<MemgraphClient>,
}

/// Create playground state. Returns None if Memgraph is unreachable.
pub async fn create_playground_state() -> Option<PlaygroundState> {
    let config = MemgraphConfig::default();
    let memgraph = Arc::new(MemgraphClient::connect(&config).await.ok()?);
    let sets = Arc::new(SetsService::new(memgraph.clone()));
    Some(PlaygroundState { sets, memgraph })
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphNodeDto {
    pub id: i64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphEdgeDto {
    pub id: i64,
    pub from_id: i64,
    pub to_id: i64,
    pub rel_type: String,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct SubgraphDto {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphSnapshotMetaDto {
    pub truncated: bool,
    pub node_limit: u32,
    pub edge_limit: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphSnapshotDto {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
    pub meta: GraphSnapshotMetaDto,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SetDefinitionDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub description: String,
    #[serde(default)]
    pub extends: Vec<String>,
    #[serde(default)]
    pub selectors: SetSelectors,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub operations: Vec<sets::SetOperation>,
    pub cost_limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResolvedSetDto {
    pub set_id: String,
    pub set_kind: Option<String>,
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
    #[serde(default)]
    pub composition_trace: Vec<String>,
    pub completeness: Option<String>,
    #[serde(default)]
    pub degradations: Vec<String>,
    pub cost_estimate: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct SetExportDto {
    pub format: String,
    pub structured: serde_json::Value,
    pub text: String,
}

#[derive(Deserialize)]
pub struct CreateNodeRequest {
    pub labels: Vec<String>,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateEdgeRequest {
    pub from_id: i64,
    pub to_id: i64,
    pub rel_type: String,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
pub struct GetSubgraphRequest {
    pub node_ids: Vec<i64>,
}

#[derive(Deserialize)]
pub struct GetGraphRequest {
    pub node_limit: Option<u32>,
    pub edge_limit: Option<u32>,
}

#[derive(Deserialize)]
pub struct ExportRequest {
    pub format: String,
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub usage_hint: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub included: Vec<String>,
    #[serde(default)]
    pub excluded: Option<Vec<String>>,
    pub resolved: ResolvedSetDto,
}

#[derive(Deserialize)]
pub struct CreateSetFromSelectionRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub kind: Option<String>,
    pub node_ids: Vec<i64>,
    #[serde(default)]
    pub cost_limit: Option<u32>,
}

fn node_to_dto(node: &memgraph::GraphNode) -> GraphNodeDto {
    GraphNodeDto {
        id: node.id,
        labels: node.labels.clone(),
        properties: node.properties.clone(),
    }
}

fn edge_to_dto(edge: &memgraph::GraphEdge) -> GraphEdgeDto {
    GraphEdgeDto {
        id: edge.id,
        from_id: edge.from_id,
        to_id: edge.to_id,
        rel_type: edge.rel_type.clone(),
        properties: edge.properties.clone(),
    }
}

fn normalize_limit(requested: Option<u32>, default: u32, max: u32) -> u32 {
    requested.unwrap_or(default).clamp(1, max)
}

fn snapshot_to_dto(snapshot: GraphSnapshot) -> GraphSnapshotDto {
    GraphSnapshotDto {
        nodes: snapshot.subgraph.nodes.iter().map(node_to_dto).collect(),
        edges: snapshot.subgraph.edges.iter().map(edge_to_dto).collect(),
        meta: GraphSnapshotMetaDto {
            truncated: snapshot.truncated,
            node_limit: snapshot.node_limit,
            edge_limit: snapshot.edge_limit,
        },
    }
}

fn kind_from_string(kind: &str) -> sets::SetKind {
    match kind {
        "boundary" => sets::SetKind::Boundary,
        "projection" => sets::SetKind::Projection,
        _ => sets::SetKind::Semantic,
    }
}

fn kind_to_string(kind: &sets::SetKind) -> String {
    match kind {
        sets::SetKind::Semantic => "semantic",
        sets::SetKind::Boundary => "boundary",
        sets::SetKind::Projection => "projection",
    }
    .to_string()
}

fn set_definition_from_dto(
    route_id: Option<&str>,
    definition: SetDefinitionDto,
) -> Result<SetDefinition, String> {
    if let Some(route_id) = route_id {
        if route_id != definition.id {
            return Err(format!(
                "route id '{route_id}' does not match set id '{}'",
                definition.id
            ));
        }
    }

    Ok(SetDefinition {
        id: definition.id,
        name: definition.name,
        kind: kind_from_string(&definition.kind),
        description: definition.description,
        extends: definition.extends,
        selectors: definition.selectors,
        filters: definition.filters,
        operations: definition.operations,
        cost_limit: definition.cost_limit,
    })
}

fn set_definition_to_dto(definition: SetDefinition) -> SetDefinitionDto {
    SetDefinitionDto {
        id: definition.id,
        name: definition.name,
        kind: kind_to_string(&definition.kind),
        description: definition.description,
        extends: definition.extends,
        selectors: definition.selectors,
        filters: definition.filters,
        operations: definition.operations,
        cost_limit: definition.cost_limit,
    }
}

fn resolved_set_to_dto(resolved: ResolvedSet) -> ResolvedSetDto {
    ResolvedSetDto {
        set_id: resolved.set_id,
        set_kind: resolved.set_kind,
        nodes: resolved.nodes.iter().map(node_to_dto).collect(),
        edges: resolved.edges.iter().map(edge_to_dto).collect(),
        composition_trace: resolved.composition_trace,
        completeness: resolved.completeness,
        degradations: resolved.degradations,
        cost_estimate: resolved.cost_estimate,
    }
}

fn dto_to_resolved_set(dto: &ResolvedSetDto) -> ResolvedSet {
    ResolvedSet {
        set_id: dto.set_id.clone(),
        set_kind: dto.set_kind.clone(),
        nodes: dto
            .nodes
            .iter()
            .map(|node| memgraph::GraphNode {
                id: node.id,
                labels: node.labels.clone(),
                properties: node.properties.clone(),
            })
            .collect(),
        edges: dto
            .edges
            .iter()
            .map(|edge| memgraph::GraphEdge {
                id: edge.id,
                from_id: edge.from_id,
                to_id: edge.to_id,
                rel_type: edge.rel_type.clone(),
                properties: edge.properties.clone(),
            })
            .collect(),
        composition_trace: dto.composition_trace.clone(),
        completeness: dto.completeness.clone(),
        degradations: dto.degradations.clone(),
        cost_estimate: dto.cost_estimate,
    }
}

fn root_set_definition() -> SetDefinitionDto {
    SetDefinitionDto {
        id: ROOT_SET_ID.to_string(),
        name: "Root Set".to_string(),
        kind: "boundary".to_string(),
        description: "Synthetic root set exposing the widest bounded graph scope available to the playground.".to_string(),
        extends: Vec::new(),
        selectors: SetSelectors::default(),
        filters: Vec::new(),
        operations: Vec::new(),
        cost_limit: None,
    }
}

fn root_resolved_set(snapshot: GraphSnapshotDto) -> ResolvedSetDto {
    let mut degradations = Vec::new();
    if snapshot.meta.truncated {
        degradations.push("root set truncated by gateway graph limits".to_string());
    }

    ResolvedSetDto {
        set_id: ROOT_SET_ID.to_string(),
        set_kind: Some("boundary".to_string()),
        nodes: snapshot.nodes.clone(),
        edges: snapshot.edges.clone(),
        composition_trace: vec!["resolve root".to_string()],
        completeness: Some(
            if snapshot.meta.truncated {
                "degraded"
            } else {
                "full"
            }
            .to_string(),
        ),
        degradations,
        cost_estimate: Some((snapshot.nodes.len() + snapshot.edges.len()) as u64),
    }
}

fn root_limits(bound: &BoundSet) -> (u32, u32) {
    let node_limit = bound
        .parameters
        .get("node_limit")
        .and_then(|value| value.as_u64())
        .map(|value| value as u32);
    let edge_limit = bound
        .parameters
        .get("edge_limit")
        .and_then(|value| value.as_u64())
        .map(|value| value as u32);

    (
        normalize_limit(node_limit, DEFAULT_GRAPH_NODE_LIMIT, MAX_GRAPH_NODE_LIMIT),
        normalize_limit(edge_limit, DEFAULT_GRAPH_EDGE_LIMIT, MAX_GRAPH_EDGE_LIMIT),
    )
}

fn slugify_set_name(name: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "set".to_string()
    } else {
        slug
    }
}

pub async fn handle_get_graph(
    State(state): State<AppState>,
    Query(query): Query<GetGraphRequest>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    let node_limit = normalize_limit(
        query.node_limit,
        DEFAULT_GRAPH_NODE_LIMIT,
        MAX_GRAPH_NODE_LIMIT,
    );
    let edge_limit = normalize_limit(
        query.edge_limit,
        DEFAULT_GRAPH_EDGE_LIMIT,
        MAX_GRAPH_EDGE_LIMIT,
    );

    match pg.memgraph.get_graph_snapshot(node_limit, edge_limit).await {
        Ok(snapshot) => (
            StatusCode::OK,
            Json(serde_json::json!(snapshot_to_dto(snapshot))),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_create_node(
    State(state): State<AppState>,
    Json(req): Json<CreateNodeRequest>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable (Memgraph not connected)"})),
        )
            .into_response();
    };

    match pg.memgraph.create_node(&req.labels, &req.properties).await {
        Ok(node) => (
            StatusCode::CREATED,
            Json(serde_json::json!(node_to_dto(&node))),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_create_edge(
    State(state): State<AppState>,
    Json(req): Json<CreateEdgeRequest>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    match pg
        .memgraph
        .create_edge(req.from_id, req.to_id, &req.rel_type, &req.properties)
        .await
    {
        Ok(edge) => (
            StatusCode::CREATED,
            Json(serde_json::json!(edge_to_dto(&edge))),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_get_subgraph(
    State(state): State<AppState>,
    Json(req): Json<GetSubgraphRequest>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    match pg.memgraph.get_subgraph(&req.node_ids).await {
        Ok(subgraph) => (
            StatusCode::OK,
            Json(serde_json::json!(SubgraphDto {
                nodes: subgraph.nodes.iter().map(node_to_dto).collect(),
                edges: subgraph.edges.iter().map(edge_to_dto).collect(),
            })),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_inspect_node(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    match pg.memgraph.inspect_node(id).await {
        Ok(Some(node)) => {
            (StatusCode::OK, Json(serde_json::json!(node_to_dto(&node)))).into_response()
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "node not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_create(
    State(state): State<AppState>,
    Json(definition): Json<SetDefinitionDto>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    let definition = match set_definition_from_dto(None, definition) {
        Ok(definition) => definition,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": error})),
            )
                .into_response();
        }
    };

    match pg.sets.create_set(definition).await {
        Ok(()) => (StatusCode::CREATED, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(definition): Json<SetDefinitionDto>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    let definition = match set_definition_from_dto(Some(&id), definition) {
        Ok(definition) => definition,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": error})),
            )
                .into_response();
        }
    };

    match pg.sets.update_set(definition).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_bind(
    State(state): State<AppState>,
    Json(bound): Json<BoundSet>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    if bound.set_id == ROOT_SET_ID {
        return (StatusCode::OK, Json(serde_json::json!(bound))).into_response();
    }

    match pg.sets.bind(bound).await {
        Ok(bound) => (StatusCode::OK, Json(serde_json::json!(bound))).into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_resolve(
    State(state): State<AppState>,
    Json(bound): Json<BoundSet>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    if bound.set_id == ROOT_SET_ID {
        let (node_limit, edge_limit) = root_limits(&bound);
        return match pg.memgraph.get_graph_snapshot(node_limit, edge_limit).await {
            Ok(snapshot) => {
                let snapshot = snapshot_to_dto(snapshot);
                (
                    StatusCode::OK,
                    Json(serde_json::json!(root_resolved_set(snapshot))),
                )
                    .into_response()
            }
            Err(error) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": error.to_string()})),
            )
                .into_response(),
        };
    }

    match pg.sets.resolve(bound).await {
        Ok(resolved) => (
            StatusCode::OK,
            Json(serde_json::json!(resolved_set_to_dto(resolved))),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_export(
    State(_state): State<AppState>,
    Json(req): Json<ExportRequest>,
) -> impl IntoResponse {
    let resolved = dto_to_resolved_set(&req.resolved);
    let export = export_for_llm(
        &resolved,
        &req.format,
        req.purpose.as_deref(),
        &req.constraints,
        req.usage_hint.as_deref(),
        req.role.as_deref(),
        &req.included,
        req.excluded.as_deref(),
    );

    (
        StatusCode::OK,
        Json(serde_json::json!(SetExportDto {
            format: export.format,
            structured: export.structured,
            text: export.text,
        })),
    )
        .into_response()
}

pub async fn handle_sets_list(State(state): State<AppState>) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    match pg.sets.list_sets().await {
        Ok(ids) => {
            let mut definitions = vec![root_set_definition()];
            for id in ids {
                if let Ok(Some(definition)) = pg.sets.get_set(&id).await {
                    definitions.push(set_definition_to_dto(definition));
                }
            }
            (StatusCode::OK, Json(serde_json::json!(definitions))).into_response()
        }
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    if id == ROOT_SET_ID {
        return (
            StatusCode::OK,
            Json(serde_json::json!(root_set_definition())),
        )
            .into_response();
    }

    match pg.sets.get_set(&id).await {
        Ok(Some(definition)) => (
            StatusCode::OK,
            Json(serde_json::json!(set_definition_to_dto(definition))),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "set not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub async fn handle_sets_from_selection(
    State(state): State<AppState>,
    Json(req): Json<CreateSetFromSelectionRequest>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };

    if req.node_ids.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "node_ids must not be empty"})),
        )
            .into_response();
    }

    let set_id = slugify_set_name(&req.name);
    let definition = SetDefinition {
        id: set_id.clone(),
        name: req.name,
        kind: kind_from_string(req.kind.as_deref().unwrap_or("boundary")),
        description: req.description,
        extends: Vec::new(),
        selectors: SetSelectors {
            node_ids: req.node_ids,
            label: None,
            props: HashMap::new(),
        },
        filters: Vec::new(),
        operations: Vec::new(),
        cost_limit: req.cost_limit,
    };

    match pg.sets.create_set(definition).await {
        Ok(()) => match pg.sets.get_set(&set_id).await {
            Ok(Some(definition)) => (
                StatusCode::CREATED,
                Json(serde_json::json!(set_definition_to_dto(definition))),
            )
                .into_response(),
            Ok(None) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "created set could not be reloaded"})),
            )
                .into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": error.to_string()})),
            )
                .into_response(),
        },
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

pub fn playground_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};

    axum::Router::new()
        .route("/api/playground/graph", get(handle_get_graph))
        .route("/api/playground/graph/nodes", post(handle_create_node))
        .route("/api/playground/graph/edges", post(handle_create_edge))
        .route("/api/playground/graph/subgraph", post(handle_get_subgraph))
        .route("/api/playground/graph/nodes/{id}", get(handle_inspect_node))
        .route(
            "/api/playground/sets",
            get(handle_sets_list).post(handle_sets_create),
        )
        .route(
            "/api/playground/sets/{id}",
            get(handle_sets_get).put(handle_sets_update),
        )
        .route("/api/playground/sets/bind", post(handle_sets_bind))
        .route("/api/playground/sets/resolve", post(handle_sets_resolve))
        .route("/api/playground/sets/export", post(handle_sets_export))
        .route(
            "/api/playground/sets/from-selection",
            post(handle_sets_from_selection),
        )
}

#[cfg(test)]
mod tests {
    use super::{
        dto_to_resolved_set, edge_to_dto, node_to_dto, normalize_limit, root_set_definition,
        slugify_set_name, snapshot_to_dto, GraphEdgeDto, GraphNodeDto, GraphSnapshotDto,
        ResolvedSetDto, SetDefinitionDto,
    };
    use memgraph::{GraphEdge, GraphNode, GraphSnapshot, Subgraph};
    use std::collections::HashMap;

    #[test]
    fn node_and_edge_dto_mapping_preserves_graph_fields() {
        let node = GraphNode {
            id: 7,
            labels: vec!["Concept".to_string(), "Business".to_string()],
            properties: HashMap::from([(
                "name".to_string(),
                serde_json::Value::String("Pricing".to_string()),
            )]),
        };
        let edge = GraphEdge {
            id: 9,
            from_id: 7,
            to_id: 8,
            rel_type: "RELATES_TO".to_string(),
            properties: HashMap::from([(
                "weight".to_string(),
                serde_json::Value::Number(1.into()),
            )]),
        };

        let node_dto = node_to_dto(&node);
        let edge_dto = edge_to_dto(&edge);

        assert_eq!(node_dto.id, 7);
        assert_eq!(node_dto.labels, vec!["Concept", "Business"]);
        assert_eq!(node_dto.properties["name"], "Pricing");
        assert_eq!(edge_dto.id, 9);
        assert_eq!(edge_dto.from_id, 7);
        assert_eq!(edge_dto.to_id, 8);
        assert_eq!(edge_dto.rel_type, "RELATES_TO");
        assert_eq!(edge_dto.properties["weight"], 1);
    }

    #[test]
    fn dto_to_resolved_set_round_trips_public_payload_into_domain_type() {
        let resolved = dto_to_resolved_set(&ResolvedSetDto {
            set_id: "business_logic_shared".to_string(),
            set_kind: Some("projection".to_string()),
            nodes: vec![GraphNodeDto {
                id: 1,
                labels: vec!["Concept".to_string()],
                properties: HashMap::from([(
                    "name".to_string(),
                    serde_json::Value::String("Pricing".to_string()),
                )]),
            }],
            edges: vec![GraphEdgeDto {
                id: 10,
                from_id: 1,
                to_id: 2,
                rel_type: "RELATES_TO".to_string(),
                properties: HashMap::new(),
            }],
            composition_trace: vec!["resolve business_logic_shared".to_string()],
            completeness: Some("full".to_string()),
            degradations: vec!["cost_limit applied (4)".to_string()],
            cost_estimate: Some(3),
        });

        assert_eq!(resolved.set_id, "business_logic_shared");
        assert_eq!(resolved.set_kind.as_deref(), Some("projection"));
        assert_eq!(resolved.nodes[0].properties["name"], "Pricing");
        assert_eq!(resolved.edges[0].rel_type, "RELATES_TO");
        assert_eq!(resolved.composition_trace.len(), 1);
        assert_eq!(resolved.degradations[0], "cost_limit applied (4)");
        assert_eq!(resolved.cost_estimate, Some(3));
    }

    #[test]
    fn snapshot_to_dto_preserves_graph_and_meta() {
        let dto: GraphSnapshotDto = snapshot_to_dto(GraphSnapshot {
            subgraph: Subgraph {
                nodes: vec![GraphNode {
                    id: 1,
                    labels: vec!["Concept".to_string()],
                    properties: HashMap::new(),
                }],
                edges: vec![GraphEdge {
                    id: 2,
                    from_id: 1,
                    to_id: 1,
                    rel_type: "SELF".to_string(),
                    properties: HashMap::new(),
                }],
            },
            truncated: true,
            node_limit: 100,
            edge_limit: 250,
        });

        assert_eq!(dto.nodes.len(), 1);
        assert_eq!(dto.edges.len(), 1);
        assert!(dto.meta.truncated);
        assert_eq!(dto.meta.node_limit, 100);
        assert_eq!(dto.meta.edge_limit, 250);
    }

    #[test]
    fn normalize_limit_clamps_requested_values() {
        assert_eq!(normalize_limit(None, 200, 1000), 200);
        assert_eq!(normalize_limit(Some(0), 200, 1000), 1);
        assert_eq!(normalize_limit(Some(5000), 200, 1000), 1000);
    }

    #[test]
    fn root_set_definition_uses_set_vocabulary() {
        let root = root_set_definition();

        assert_eq!(root.id, "root");
        assert_eq!(root.name, "Root Set");
        assert_eq!(root.kind, "boundary");
        assert!(root.description.contains("Synthetic root set"));
    }

    #[test]
    fn slugify_set_name_creates_stable_ids() {
        assert_eq!(slugify_set_name("Pricing + Margin"), "pricing-margin");
        assert_eq!(slugify_set_name("!!!"), "set");
    }

    #[test]
    fn set_definition_dto_shape_is_serializable() {
        let dto = SetDefinitionDto {
            id: "shared-business".to_string(),
            name: "Shared business".to_string(),
            kind: "projection".to_string(),
            description: "Shared business logic".to_string(),
            extends: vec!["core".to_string()],
            selectors: sets::SetSelectors::default(),
            filters: vec!["label:Business".to_string()],
            operations: Vec::new(),
            cost_limit: Some(12),
        };

        let value = serde_json::to_value(dto).expect("set definition dto should serialize");
        assert_eq!(value["kind"], "projection");
        assert_eq!(value["id"], "shared-business");
    }
}
