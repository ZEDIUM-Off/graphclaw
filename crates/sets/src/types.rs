use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Set kind (v0 typology).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SetKind {
    Semantic,
    Boundary,
    Projection,
}

/// Set selector: how to get initial nodes (v0: by ids or by label).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SetSelectors {
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
pub enum SetOperation {
    Union {
        set_ids: Vec<String>,
    },
    Intersection {
        set_ids: Vec<String>,
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

/// SetDefinition: reusable definition (id, name, kind, selectors, operations, etc.).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetDefinition {
    pub id: String,
    pub name: String,
    pub kind: SetKind,
    pub description: String,
    #[serde(default)]
    pub extends: Vec<String>,
    #[serde(default)]
    pub selectors: SetSelectors,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub operations: Vec<SetOperation>,
    #[serde(default)]
    pub cost_limit: Option<u32>,
}

/// BoundSet: set definition instantiated with anchors and parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundSet {
    pub set_id: String,
    #[serde(default)]
    pub anchors: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    pub resolution_scope: Option<String>,
}

/// ResolvedSet: materialised result on the graph.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResolvedSet {
    pub set_id: String,
    #[serde(default)]
    pub set_kind: Option<String>,
    pub nodes: Vec<memgraph::GraphNode>,
    pub edges: Vec<memgraph::GraphEdge>,
    #[serde(default)]
    pub composition_trace: Vec<String>,
    pub completeness: Option<String>,
    #[serde(default)]
    pub degradations: Vec<String>,
    pub cost_estimate: Option<u64>,
}

/// SetExport: export for LLM (format + structured + text).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetExport {
    pub format: String,
    pub structured: serde_json::Value,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::{
        BoundSet, ResolvedSet, SetDefinition, SetExport, SetKind, SetOperation, SetSelectors,
    };
    use memgraph::{GraphEdge, GraphNode};
    use std::collections::HashMap;

    #[test]
    fn set_definition_round_trips_with_operations() {
        let definition = SetDefinition {
            id: "shared_business".to_string(),
            name: "Shared business".to_string(),
            kind: SetKind::Boundary,
            description: "Shared business logic".to_string(),
            extends: vec!["set_highfinity_core".to_string()],
            selectors: SetSelectors {
                node_ids: vec![1, 2],
                label: Some("Business".to_string()),
                props: HashMap::from([(
                    "workspace".to_string(),
                    serde_json::Value::String("HIGHFINITY".to_string()),
                )]),
            },
            filters: vec!["label:Business".to_string()],
            operations: vec![
                SetOperation::Expand {
                    relation_type: Some("RELATES_TO".to_string()),
                    depth: 2,
                },
                SetOperation::Slice {
                    limit: 8,
                    order: Some("name_asc".to_string()),
                },
            ],
            cost_limit: Some(12),
        };

        let json = serde_json::to_value(&definition).expect("definition should serialize");
        let decoded: SetDefinition =
            serde_json::from_value(json).expect("definition should deserialize");

        assert_eq!(decoded.id, definition.id);
        assert_eq!(decoded.extends, definition.extends);
        assert_eq!(decoded.selectors.node_ids, vec![1, 2]);
        assert_eq!(decoded.selectors.label.as_deref(), Some("Business"));
        assert!(matches!(
            decoded.operations[0],
            SetOperation::Expand {
                relation_type: Some(_),
                depth: 2
            }
        ));
    }

    #[test]
    fn bound_set_deserializes_missing_maps_to_defaults() {
        let decoded: BoundSet = serde_json::from_value(serde_json::json!({
            "set_id": "set_business",
            "resolution_scope": "playground"
        }))
        .expect("bound set should deserialize");

        assert_eq!(decoded.set_id, "set_business");
        assert!(decoded.anchors.is_empty());
        assert!(decoded.parameters.is_empty());
        assert_eq!(decoded.resolution_scope.as_deref(), Some("playground"));
    }

    #[test]
    fn resolved_set_export_holds_structured_and_text_payloads() {
        let resolved = ResolvedSet {
            set_id: "business_logic_shared".to_string(),
            set_kind: Some("projection".to_string()),
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
        let export = SetExport {
            format: "llm_compact".to_string(),
            structured: serde_json::json!({
                "set_id": resolved.set_id,
                "nodes": ["Pricing"],
            }),
            text: "set_id: business_logic_shared".to_string(),
        };

        assert_eq!(export.format, "llm_compact");
        assert_eq!(export.structured["set_id"], "business_logic_shared");
        assert!(export.text.contains("set_id: business_logic_shared"));
    }
}
