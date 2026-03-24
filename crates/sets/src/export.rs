//! LLM export: llm_compact (YAML-like) and llm_explained (narrative Markdown).

use crate::types::{ResolvedSet, SetExport};
use serde_json::json;

/// Produce compact YAML-like export for injection into LLM context.
pub fn export_llm_compact(
    resolved: &ResolvedSet,
    purpose: Option<&str>,
    constraints: &[String],
    usage_hint: Option<&str>,
) -> (serde_json::Value, String) {
    let node_names: Vec<String> = resolved
        .nodes
        .iter()
        .filter_map(|n| {
            n.properties
                .get("name")
                .and_then(|v| v.as_str())
                .map(String::from)
        })
        .collect();
    let node_names = if node_names.is_empty() {
        resolved
            .nodes
            .iter()
            .map(|n| {
                n.labels
                    .first()
                    .cloned()
                    .unwrap_or_else(|| n.id.to_string())
            })
            .collect::<Vec<_>>()
    } else {
        node_names
    };
    let edges_text: Vec<String> = resolved
        .edges
        .iter()
        .map(|e| {
            let from_label = resolved
                .nodes
                .iter()
                .find(|n| n.id == e.from_id)
                .and_then(|n| {
                    n.properties
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                })
                .or_else(|| {
                    resolved
                        .nodes
                        .iter()
                        .find(|n| n.id == e.from_id)
                        .and_then(|n| n.labels.first().cloned())
                })
                .unwrap_or_else(|| e.from_id.to_string());
            let to_label = resolved
                .nodes
                .iter()
                .find(|n| n.id == e.to_id)
                .and_then(|n| {
                    n.properties
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                })
                .or_else(|| {
                    resolved
                        .nodes
                        .iter()
                        .find(|n| n.id == e.to_id)
                        .and_then(|n| n.labels.first().cloned())
                })
                .unwrap_or_else(|| e.to_id.to_string());
            format!("{} {} {}", from_label, e.rel_type, to_label)
        })
        .collect();
    let kind_str = resolved.set_kind.as_deref().unwrap_or("projection");
    let structured = json!({
        "set_id": resolved.set_id,
        "set_kind": kind_str,
        "purpose": purpose.unwrap_or(""),
        "nodes": node_names,
        "edges": edges_text,
        "constraints": constraints,
        "usage_hint": usage_hint.unwrap_or(""),
        "composition_trace": resolved.composition_trace,
        "degradations": resolved.degradations,
    });
    let mut yaml = format!(
        "set_id: {}\nset_kind: {}\npurpose: {}\nnodes:\n",
        resolved.set_id,
        kind_str,
        purpose.unwrap_or("")
    );
    for n in &node_names {
        yaml.push_str(&format!("  - {}\n", n));
    }
    yaml.push_str("edges:\n");
    for e in &edges_text {
        yaml.push_str(&format!("  - {}\n", e));
    }
    yaml.push_str("constraints:\n");
    for c in constraints {
        yaml.push_str(&format!("  - {}\n", c));
    }
    if let Some(h) = usage_hint {
        yaml.push_str(&format!("usage_hint: {}\n", h));
    }
    if !resolved.composition_trace.is_empty() {
        yaml.push_str("composition_trace:\n");
        for step in &resolved.composition_trace {
            yaml.push_str(&format!("  - {}\n", step));
        }
    }
    if !resolved.degradations.is_empty() {
        yaml.push_str("degradations:\n");
        for degradation in &resolved.degradations {
            yaml.push_str(&format!("  - {}\n", degradation));
        }
    }
    (structured, yaml)
}

/// Produce narrative Markdown export for documentation and discussion.
pub fn export_llm_explained(
    resolved: &ResolvedSet,
    role: Option<&str>,
    included: &[String],
    excluded: Option<&[String]>,
) -> (serde_json::Value, String) {
    let node_names: Vec<String> = resolved
        .nodes
        .iter()
        .filter_map(|n| {
            n.properties
                .get("name")
                .and_then(|v| v.as_str())
                .map(String::from)
        })
        .collect();
    let node_names = if node_names.is_empty() {
        resolved
            .nodes
            .iter()
            .map(|n| {
                n.labels
                    .first()
                    .cloned()
                    .unwrap_or_else(|| n.id.to_string())
            })
            .collect::<Vec<_>>()
    } else {
        node_names
    };
    let included_list = if included.is_empty() {
        node_names.clone()
    } else {
        included.to_vec()
    };
    let edges_text: Vec<String> = resolved
        .edges
        .iter()
        .map(|e| {
            let from_label = resolved
                .nodes
                .iter()
                .find(|n| n.id == e.from_id)
                .and_then(|n| {
                    n.properties
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                })
                .unwrap_or_else(|| e.from_id.to_string());
            let to_label = resolved
                .nodes
                .iter()
                .find(|n| n.id == e.to_id)
                .and_then(|n| {
                    n.properties
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                })
                .unwrap_or_else(|| e.to_id.to_string());
            format!("- {} {} {}", from_label, e.rel_type, to_label)
        })
        .collect();
    let structured = json!({
        "set_id": resolved.set_id,
        "role": role.unwrap_or(""),
        "included_concepts": included_list,
        "excluded_concepts": excluded.unwrap_or(&[]),
        "relations": edges_text,
        "composition_trace": resolved.composition_trace,
        "degradations": resolved.degradations,
    });
    let mut md = format!(
        "# Set Export — {}\n\n## Role\n{}\n\n## Included concepts\n",
        resolved.set_id,
        role.unwrap_or("")
    );
    for n in &included_list {
        md.push_str(&format!("- {}\n", n));
    }
    if let Some(ex) = excluded {
        md.push_str("\n## Excluded concepts\n");
        for n in ex {
            md.push_str(&format!("- {}\n", n));
        }
    }
    md.push_str("\n## Relations\n");
    for e in &edges_text {
        md.push_str(&format!("{}\n", e));
    }
    if !resolved.composition_trace.is_empty() {
        md.push_str("\n## Composition trace\n");
        for step in &resolved.composition_trace {
            md.push_str(&format!("- {}\n", step));
        }
    }
    if !resolved.degradations.is_empty() {
        md.push_str("\n## Degradations\n");
        for degradation in &resolved.degradations {
            md.push_str(&format!("- {}\n", degradation));
        }
    }
    (structured, md)
}

/// Build a SetExport for the given format.
pub fn export_for_llm(
    resolved: &ResolvedSet,
    format: &str,
    purpose: Option<&str>,
    constraints: &[String],
    usage_hint: Option<&str>,
    role: Option<&str>,
    included: &[String],
    excluded: Option<&[String]>,
) -> SetExport {
    let (structured, text) = match format {
        "llm_explained" => export_llm_explained(resolved, role, included, excluded),
        _ => export_llm_compact(resolved, purpose, constraints, usage_hint),
    };
    SetExport {
        format: format.to_string(),
        structured,
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::{export_llm_compact, export_llm_explained};
    use crate::ResolvedSet;
    use memgraph::{GraphEdge, GraphNode};
    use std::collections::HashMap;

    fn sample_resolved_set() -> ResolvedSet {
        ResolvedSet {
            set_id: "business_logic_shared".to_string(),
            set_kind: Some("projection".to_string()),
            nodes: vec![
                GraphNode {
                    id: 1,
                    labels: vec!["Concept".to_string()],
                    properties: HashMap::from([(
                        "name".to_string(),
                        serde_json::Value::String("Pricing".to_string()),
                    )]),
                },
                GraphNode {
                    id: 2,
                    labels: vec!["Concept".to_string()],
                    properties: HashMap::from([(
                        "name".to_string(),
                        serde_json::Value::String("Margin".to_string()),
                    )]),
                },
            ],
            edges: vec![GraphEdge {
                id: 10,
                from_id: 1,
                to_id: 2,
                rel_type: "RELATES_TO".to_string(),
                properties: HashMap::new(),
            }],
            composition_trace: vec!["resolve business_logic_shared".to_string()],
            completeness: Some("full".to_string()),
            degradations: vec!["cost_limit applied (3)".to_string()],
            cost_estimate: Some(3),
        }
    }

    #[test]
    fn compact_export_includes_trace_and_degradations() {
        let (_, text) = export_llm_compact(
            &sample_resolved_set(),
            Some("Shared business logic"),
            &["business-only".to_string()],
            Some("Use for business reasoning."),
        );

        assert!(text.contains("composition_trace:"));
        assert!(text.contains("degradations:"));
        assert!(text.contains("Pricing RELATES_TO Margin"));
        assert!(text.contains("set_id: business_logic_shared"));
    }

    #[test]
    fn explained_export_includes_sections_for_trace_and_degradations() {
        let (_, text) = export_llm_explained(
            &sample_resolved_set(),
            Some("Shared business logic between workspaces."),
            &[],
            Some(&["Brand-specific concepts".to_string()]),
        );

        assert!(text.contains("# Set Export"));
        assert!(text.contains("## Composition trace"));
        assert!(text.contains("## Degradations"));
        assert!(text.contains("## Excluded concepts"));
    }
}
