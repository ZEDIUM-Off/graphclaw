use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// View kind (v0 typology).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ViewKind {
    Semantic,
    Boundary,
    Projection,
}

/// Selector: how to get initial nodes (v0: by ids or by label).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Selectors {
    /// Initial node ids (extensional).
    #[serde(default)]
    pub node_ids: Vec<i64>,
    /// Optional label filter (e.g. "Concept").
    #[serde(default)]
    pub label: Option<String>,
    /// Optional property filter for label (e.g. workspace: "HIGHFINITY").
    #[serde(default)]
    pub props: HashMap<String, serde_json::Value>,
}

/// Single composition/transform step.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ViewOperation {
    Union {
        view_ids: Vec<String>,
    },
    Intersection {
        view_ids: Vec<String>,
    },
    Difference {
        a: String,
        b: String,
    },
    Expand {
        relation_type: Option<String>,
        depth: u32,
    },
    FilterNodes {
        predicate: String,
    },
    FilterEdges {
        predicate: String,
    },
    Project {
        mode: String,
    },
    Slice {
        limit: u32,
        order: Option<String>,
    },
}

/// ViewTemplate: reusable definition (id, name, kind, selectors, operations, etc.).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViewTemplate {
    pub id: String,
    pub name: String,
    pub kind: ViewKind,
    pub description: String,
    #[serde(default)]
    pub extends: Vec<String>,
    #[serde(default)]
    pub selectors: Selectors,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub operations: Vec<ViewOperation>,
    #[serde(default)]
    pub cost_limit: Option<u32>,
}

/// BoundView: template instantiated with anchors and parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundView {
    pub template_id: String,
    #[serde(default)]
    pub anchors: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    pub resolution_scope: Option<String>,
}

/// ResolvedView: materialised result on the graph.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResolvedView {
    pub view_id: String,
    #[serde(default)]
    pub view_kind: Option<String>,
    pub nodes: Vec<memgraph::GraphNode>,
    pub edges: Vec<memgraph::GraphEdge>,
    #[serde(default)]
    pub composition_trace: Vec<String>,
    pub completeness: Option<String>,
    #[serde(default)]
    pub degradations: Vec<String>,
    pub cost_estimate: Option<u64>,
}

/// ResolvedViewExport: export for LLM (format + structured + text).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolvedViewExport {
    pub format: String,
    pub structured: serde_json::Value,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::{
        BoundView, ResolvedView, ResolvedViewExport, Selectors, ViewKind, ViewOperation,
        ViewTemplate,
    };
    use memgraph::{GraphEdge, GraphNode};
    use std::collections::HashMap;

    #[test]
    fn view_template_round_trips_with_operations() {
        let template = ViewTemplate {
            id: "shared_business".to_string(),
            name: "Shared business".to_string(),
            kind: ViewKind::Boundary,
            description: "Shared business logic".to_string(),
            extends: vec!["view_highfinity_core".to_string()],
            selectors: Selectors {
                node_ids: vec![1, 2],
                label: Some("Business".to_string()),
                props: HashMap::from([(
                    "workspace".to_string(),
                    serde_json::Value::String("HIGHFINITY".to_string()),
                )]),
            },
            filters: vec!["label:Business".to_string()],
            operations: vec![
                ViewOperation::Expand {
                    relation_type: Some("RELATES_TO".to_string()),
                    depth: 2,
                },
                ViewOperation::Slice {
                    limit: 8,
                    order: Some("name_asc".to_string()),
                },
            ],
            cost_limit: Some(12),
        };

        let json = serde_json::to_value(&template).expect("template should serialize");
        let decoded: ViewTemplate =
            serde_json::from_value(json).expect("template should deserialize");

        assert_eq!(decoded.id, template.id);
        assert_eq!(decoded.extends, template.extends);
        assert_eq!(decoded.selectors.node_ids, vec![1, 2]);
        assert_eq!(decoded.selectors.label.as_deref(), Some("Business"));
        assert!(matches!(
            decoded.operations[0],
            ViewOperation::Expand {
                relation_type: Some(_),
                depth: 2
            }
        ));
    }

    #[test]
    fn bound_view_deserializes_missing_maps_to_defaults() {
        let decoded: BoundView = serde_json::from_value(serde_json::json!({
            "template_id": "view_business",
            "resolution_scope": "playground"
        }))
        .expect("bound view should deserialize");

        assert_eq!(decoded.template_id, "view_business");
        assert!(decoded.anchors.is_empty());
        assert!(decoded.parameters.is_empty());
        assert_eq!(decoded.resolution_scope.as_deref(), Some("playground"));
    }

    #[test]
    fn resolved_view_export_holds_structured_and_text_payloads() {
        let resolved = ResolvedView {
            view_id: "business_logic_shared".to_string(),
            view_kind: Some("projection".to_string()),
            nodes: vec![GraphNode {
                id: 1,
                labels: vec!["Concept".to_string()],
                properties: HashMap::from([(
                    "name".to_string(),
                    serde_json::Value::String("Pricing".to_string()),
                )]),
            }],
            edges: vec![GraphEdge {
                id: 10,
                from_id: 1,
                to_id: 1,
                rel_type: "SELF".to_string(),
                properties: HashMap::new(),
            }],
            composition_trace: vec!["resolve business_logic_shared".to_string()],
            completeness: Some("full".to_string()),
            degradations: vec![],
            cost_estimate: Some(2),
        };
        let export = ResolvedViewExport {
            format: "llm_compact".to_string(),
            structured: serde_json::json!({
                "view_id": resolved.view_id,
                "nodes": ["Pricing"],
            }),
            text: "view_id: business_logic_shared".to_string(),
        };

        assert_eq!(export.format, "llm_compact");
        assert_eq!(export.structured["view_id"], "business_logic_shared");
        assert!(export.text.contains("view_id: business_logic_shared"));
    }
}
