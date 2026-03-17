//! Playground API: graph primitives and view composition/resolution/export.
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
use std::collections::HashMap;
use std::sync::Arc;
use views::{export_for_llm, BoundView, ViewTemplate, ViewsService};

const DEFAULT_GRAPH_NODE_LIMIT: u32 = 200;
const DEFAULT_GRAPH_EDGE_LIMIT: u32 = 400;
const MAX_GRAPH_NODE_LIMIT: u32 = 1_000;
const MAX_GRAPH_EDGE_LIMIT: u32 = 5_000;

/// Playground state (views service; holds Memgraph client internally).
#[derive(Clone)]
pub struct PlaygroundState {
    pub views: Arc<ViewsService>,
    pub memgraph: Arc<MemgraphClient>,
}

/// Create playground state. Returns None if Memgraph is unreachable.
pub async fn create_playground_state() -> Option<PlaygroundState> {
    let config = MemgraphConfig::default();
    let memgraph = Arc::new(MemgraphClient::connect(&config).await.ok()?);
    let views = Arc::new(ViewsService::new(memgraph.clone()));
    Some(PlaygroundState { views, memgraph })
}

// ═══════════════════════════════════════════════════════════════════════════
// DTOs (public surface; do not expose internal types)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Serialize, Deserialize)]
pub struct GraphNodeDto {
    pub id: i64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct GraphSnapshotMetaDto {
    pub truncated: bool,
    pub node_limit: u32,
    pub edge_limit: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GraphSnapshotDto {
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
    pub meta: GraphSnapshotMetaDto,
}

#[derive(Serialize, Deserialize)]
pub struct ViewTemplateDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub description: String,
    #[serde(default)]
    pub extends: Vec<String>,
    #[serde(default)]
    pub selectors: views::Selectors,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub operations: Vec<views::ViewOperation>,
    pub cost_limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ResolvedViewDto {
    pub view_id: String,
    pub view_kind: Option<String>,
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
pub struct ResolvedViewExportDto {
    pub format: String,
    pub structured: serde_json::Value,
    pub text: String,
}

// Request bodies
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
    pub format: String, // "llm_compact" | "llm_explained"
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
    /// Resolved view to export (from resolve endpoint)
    pub resolved: ResolvedViewDto,
}

// ═══════════════════════════════════════════════════════════════════════════
// Handlers (require PlaygroundState in app state; see gateway/mod.rs)
// ═══════════════════════════════════════════════════════════════════════════

fn node_to_dto(n: &memgraph::GraphNode) -> GraphNodeDto {
    GraphNodeDto {
        id: n.id,
        labels: n.labels.clone(),
        properties: n.properties.clone(),
    }
}

fn edge_to_dto(e: &memgraph::GraphEdge) -> GraphEdgeDto {
    GraphEdgeDto {
        id: e.id,
        from_id: e.from_id,
        to_id: e.to_id,
        rel_type: e.rel_type.clone(),
        properties: e.properties.clone(),
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

fn view_template_from_dto(
    route_id: Option<&str>,
    template: ViewTemplateDto,
) -> Result<ViewTemplate, String> {
    if let Some(route_id) = route_id {
        if route_id != template.id {
            return Err(format!(
                "route id '{route_id}' does not match template id '{}'",
                template.id
            ));
        }
    }

    let kind = match template.kind.as_str() {
        "boundary" => views::ViewKind::Boundary,
        "projection" => views::ViewKind::Projection,
        _ => views::ViewKind::Semantic,
    };

    Ok(ViewTemplate {
        id: template.id,
        name: template.name,
        kind,
        description: template.description,
        extends: template.extends,
        selectors: template.selectors,
        filters: template.filters,
        operations: template.operations,
        cost_limit: template.cost_limit,
    })
}

/// GET /api/playground/graph — get a bounded snapshot of the graph
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
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// POST /api/playground/graph/nodes — create a node
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
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// POST /api/playground/graph/edges — create an edge
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
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// POST /api/playground/graph/subgraph — get subgraph by node ids
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
        Ok(sg) => (
            StatusCode::OK,
            Json(serde_json::json!(SubgraphDto {
                nodes: sg.nodes.iter().map(node_to_dto).collect(),
                edges: sg.edges.iter().map(edge_to_dto).collect(),
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// GET /api/playground/graph/nodes/:id — inspect a node
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
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// POST /api/playground/views — create a view template
pub async fn handle_views_create(
    State(state): State<AppState>,
    Json(template): Json<ViewTemplateDto>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };
    let t = match view_template_from_dto(None, template) {
        Ok(template) => template,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": error})),
            )
                .into_response();
        }
    };
    match pg.views.create_template(t).await {
        Ok(()) => (StatusCode::CREATED, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// PUT /api/playground/views/:id — update a view template
pub async fn handle_views_update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(template): Json<ViewTemplateDto>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };
    let t = match view_template_from_dto(Some(&id), template) {
        Ok(template) => template,
        Err(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": error})),
            )
                .into_response();
        }
    };
    match pg.views.update_template(t).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// POST /api/playground/views/bind — bind a template (return BoundView)
pub async fn handle_views_bind(
    State(state): State<AppState>,
    Json(bound): Json<BoundView>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };
    match pg.views.bind(bound).await {
        Ok(b) => (StatusCode::OK, Json(serde_json::json!(b))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// POST /api/playground/views/resolve — resolve a bound view
pub async fn handle_views_resolve(
    State(state): State<AppState>,
    Json(bound): Json<BoundView>,
) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };
    match pg.views.resolve(bound).await {
        Ok(resolved) => (
            StatusCode::OK,
            Json(serde_json::json!(ResolvedViewDto {
                view_id: resolved.view_id,
                view_kind: resolved.view_kind,
                nodes: resolved.nodes.iter().map(node_to_dto).collect(),
                edges: resolved.edges.iter().map(edge_to_dto).collect(),
                composition_trace: resolved.composition_trace,
                completeness: resolved.completeness,
                degradations: resolved.degradations,
                cost_estimate: resolved.cost_estimate,
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

fn dto_to_resolved(dto: &ResolvedViewDto) -> views::ResolvedView {
    views::ResolvedView {
        view_id: dto.view_id.clone(),
        view_kind: dto.view_kind.clone(),
        nodes: dto
            .nodes
            .iter()
            .map(|n| memgraph::GraphNode {
                id: n.id,
                labels: n.labels.clone(),
                properties: n.properties.clone(),
            })
            .collect(),
        edges: dto
            .edges
            .iter()
            .map(|e| memgraph::GraphEdge {
                id: e.id,
                from_id: e.from_id,
                to_id: e.to_id,
                rel_type: e.rel_type.clone(),
                properties: e.properties.clone(),
            })
            .collect(),
        composition_trace: dto.composition_trace.clone(),
        completeness: dto.completeness.clone(),
        degradations: dto.degradations.clone(),
        cost_estimate: dto.cost_estimate,
    }
}

/// POST /api/playground/views/export — export a resolved view (body: resolved DTO + format options)
pub async fn handle_views_export(
    State(_state): State<AppState>,
    Json(req): Json<ExportRequest>,
) -> impl IntoResponse {
    let resolved = dto_to_resolved(&req.resolved);
    let constraints = req.constraints.clone();
    let excluded = req.excluded.clone();
    let exp = export_for_llm(
        &resolved,
        &req.format,
        req.purpose.as_deref(),
        &constraints,
        req.usage_hint.as_deref(),
        req.role.as_deref(),
        &req.included,
        excluded.as_deref(),
    );
    (
        StatusCode::OK,
        Json(serde_json::json!(ResolvedViewExportDto {
            format: exp.format,
            structured: exp.structured,
            text: exp.text,
        })),
    )
        .into_response()
}

/// GET /api/playground/views — list template ids
pub async fn handle_views_list(State(state): State<AppState>) -> impl IntoResponse {
    let Some(pg) = &state.playground else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error": "playground unavailable"})),
        )
            .into_response();
    };
    match pg.views.list_templates().await {
        Ok(ids) => (StatusCode::OK, Json(serde_json::json!(ids))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// GET /api/playground/views/:id — get a template
pub async fn handle_views_get(
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
    match pg.views.get_template(&id).await {
        Ok(Some(t)) => (
            StatusCode::OK,
            Json(serde_json::json!(ViewTemplateDto {
                id: t.id,
                name: t.name,
                kind: format!("{:?}", t.kind).to_lowercase(),
                description: t.description,
                extends: t.extends,
                selectors: t.selectors,
                filters: t.filters,
                operations: t.operations,
                cost_limit: t.cost_limit,
            })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "template not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

/// Build playground routes (to be merged into app with AppState).
pub fn playground_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/api/playground/graph", get(handle_get_graph))
        .route("/api/playground/graph/nodes", post(handle_create_node))
        .route("/api/playground/graph/edges", post(handle_create_edge))
        .route("/api/playground/graph/subgraph", post(handle_get_subgraph))
        .route("/api/playground/graph/nodes/{id}", get(handle_inspect_node))
        .route(
            "/api/playground/views",
            get(handle_views_list).post(handle_views_create),
        )
        .route("/api/playground/views/bind", post(handle_views_bind))
        .route("/api/playground/views/resolve", post(handle_views_resolve))
        .route("/api/playground/views/export", post(handle_views_export))
        .route(
            "/api/playground/views/{id}",
            get(handle_views_get).put(handle_views_update),
        )
}

#[cfg(test)]
mod tests {
    use super::{
        dto_to_resolved, edge_to_dto, node_to_dto, normalize_limit, snapshot_to_dto,
        view_template_from_dto, GraphEdgeDto, GraphNodeDto, GraphSnapshotDto, ResolvedViewDto,
        ViewTemplateDto,
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
    fn dto_to_resolved_round_trips_public_payload_into_domain_type() {
        let resolved = dto_to_resolved(&ResolvedViewDto {
            view_id: "business_logic_shared".to_string(),
            view_kind: Some("projection".to_string()),
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

        assert_eq!(resolved.view_id, "business_logic_shared");
        assert_eq!(resolved.view_kind.as_deref(), Some("projection"));
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
    fn view_template_from_dto_rejects_route_id_mismatch() {
        let template = ViewTemplateDto {
            id: "view_actual".to_string(),
            name: "Actual".to_string(),
            kind: "semantic".to_string(),
            description: "desc".to_string(),
            extends: Vec::new(),
            selectors: views::Selectors::default(),
            filters: Vec::new(),
            operations: Vec::new(),
            cost_limit: Some(5),
        };

        let error = view_template_from_dto(Some("view_path"), template)
            .expect_err("route id mismatch should fail");
        assert!(error.contains("route id 'view_path'"));
    }
}
