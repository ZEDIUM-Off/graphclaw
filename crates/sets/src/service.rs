//! SetsService: in-memory template store + bounded view resolution.

use crate::error::SetsError;
use crate::types::{BoundSet, ResolvedSet, SetDefinition, SetKind, SetOperation, SetSelectors};
use async_recursion::async_recursion;
use async_trait::async_trait;
use memgraph::{GraphEdge, GraphNode, MemgraphClient, Subgraph};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub(crate) trait GraphBackend: Send + Sync {
    async fn get_subgraph(&self, node_ids: &[i64]) -> Result<Subgraph, memgraph::MemgraphError>;
    async fn select_nodes(
        &self,
        label: Option<&str>,
        props: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<GraphNode>, memgraph::MemgraphError>;
    async fn neighborhood(
        &self,
        seed_ids: &[i64],
        relation_type: Option<&str>,
        depth: u32,
    ) -> Result<Subgraph, memgraph::MemgraphError>;
}

#[async_trait]
impl GraphBackend for MemgraphClient {
    async fn get_subgraph(&self, node_ids: &[i64]) -> Result<Subgraph, memgraph::MemgraphError> {
        MemgraphClient::get_subgraph(self, node_ids).await
    }

    async fn select_nodes(
        &self,
        label: Option<&str>,
        props: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<GraphNode>, memgraph::MemgraphError> {
        MemgraphClient::select_nodes(self, label, props).await
    }

    async fn neighborhood(
        &self,
        seed_ids: &[i64],
        relation_type: Option<&str>,
        depth: u32,
    ) -> Result<Subgraph, memgraph::MemgraphError> {
        MemgraphClient::neighborhood(self, seed_ids, relation_type, depth).await
    }
}

#[derive(Clone, Debug, Default)]
struct MaterializedSet {
    nodes: BTreeMap<i64, GraphNode>,
    edges: BTreeMap<i64, GraphEdge>,
}

impl MaterializedSet {
    fn from_subgraph(subgraph: Subgraph) -> Self {
        let mut set = Self::default();
        for node in subgraph.nodes {
            set.nodes.insert(node.id, node);
        }
        for edge in subgraph.edges {
            set.edges.insert(edge.id, edge);
        }
        set.close();
        set
    }

    fn union_with(&mut self, other: &Self) {
        for (id, node) in &other.nodes {
            self.nodes.insert(*id, node.clone());
        }
        for (id, edge) in &other.edges {
            self.edges.insert(*id, edge.clone());
        }
        self.close();
    }

    fn intersect_with(&mut self, other: &Self) {
        self.nodes.retain(|id, _| other.nodes.contains_key(id));
        self.edges.retain(|id, _| other.edges.contains_key(id));
        self.close();
    }

    fn difference_with(&mut self, other: &Self) {
        self.nodes.retain(|id, _| !other.nodes.contains_key(id));
        self.edges.retain(|id, _| !other.edges.contains_key(id));
        self.close();
    }

    fn filter_nodes<F>(&mut self, mut keep: F)
    where
        F: FnMut(&GraphNode) -> bool,
    {
        self.nodes.retain(|_, node| keep(node));
        self.close();
    }

    fn filter_edges<F>(&mut self, mut keep: F)
    where
        F: FnMut(&GraphEdge) -> bool,
    {
        self.edges.retain(|_, edge| keep(edge));
        self.close();
    }

    fn project(&mut self, mode: &str) -> Result<(), SetsError> {
        match mode {
            "identity" => {}
            "nodes_only" => self.edges.clear(),
            "named_nodes" => {
                self.filter_nodes(|node| {
                    node.properties
                        .get("name")
                        .and_then(|value| value.as_str())
                        .is_some()
                });
            }
            other => {
                return Err(SetsError::InvalidOperation(format!(
                    "unsupported project mode: {other}"
                )))
            }
        }
        Ok(())
    }

    fn slice(&mut self, limit: usize, order: Option<&str>) {
        let ordered_ids = self.ordered_node_ids(order.unwrap_or("id_asc"));
        let retained = ordered_ids.into_iter().take(limit).collect::<BTreeSet<_>>();
        self.nodes.retain(|id, _| retained.contains(id));
        self.close();
    }

    fn enforce_cost_limit(&mut self, limit: usize) {
        if limit == 0 {
            self.nodes.clear();
            self.edges.clear();
            return;
        }

        let ordered_ids = self.ordered_node_ids("id_asc");
        let retained = ordered_ids.into_iter().take(limit).collect::<BTreeSet<_>>();
        self.nodes.retain(|id, _| retained.contains(id));
        self.close();

        if self.cost() > limit {
            let edge_limit = limit.saturating_sub(self.nodes.len());
            let retained_edges = self
                .edges
                .keys()
                .copied()
                .take(edge_limit)
                .collect::<BTreeSet<_>>();
            self.edges.retain(|id, _| retained_edges.contains(id));
        }
        self.close();
    }

    fn close(&mut self) {
        let node_ids = self.nodes.keys().copied().collect::<BTreeSet<_>>();
        self.edges
            .retain(|_, edge| node_ids.contains(&edge.from_id) && node_ids.contains(&edge.to_id));
    }

    fn cost(&self) -> usize {
        self.nodes.len() + self.edges.len()
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }

    fn node_ids(&self) -> Vec<i64> {
        self.nodes.keys().copied().collect()
    }

    fn into_subgraph(self) -> Subgraph {
        Subgraph {
            nodes: self.nodes.into_values().collect(),
            edges: self.edges.into_values().collect(),
        }
    }

    fn ordered_node_ids(&self, order: &str) -> Vec<i64> {
        let mut nodes = self.nodes.values().cloned().collect::<Vec<_>>();
        match order {
            "id_desc" => nodes.sort_by(|a, b| b.id.cmp(&a.id)),
            "name_asc" => {
                nodes.sort_by(|a, b| node_name(a).cmp(&node_name(b)).then(a.id.cmp(&b.id)))
            }
            "name_desc" => {
                nodes.sort_by(|a, b| node_name(b).cmp(&node_name(a)).then(a.id.cmp(&b.id)))
            }
            "label_asc" => {
                nodes.sort_by(|a, b| first_label(a).cmp(&first_label(b)).then(a.id.cmp(&b.id)))
            }
            _ => nodes.sort_by_key(|node| node.id),
        }
        nodes.into_iter().map(|node| node.id).collect()
    }
}

#[derive(Default)]
struct ResolutionOutcome {
    set: MaterializedSet,
    trace: Vec<String>,
    degradations: Vec<String>,
}

/// Service for view templates and resolution.
pub struct SetsService {
    templates: Arc<RwLock<HashMap<String, SetDefinition>>>,
    backend: Arc<dyn GraphBackend>,
}

impl SetsService {
    /// Create a service backed by the Memgraph adapter.
    pub fn new(backend: Arc<MemgraphClient>) -> Self {
        Self::with_backend(backend)
    }

    pub(crate) fn with_backend<T>(backend: Arc<T>) -> Self
    where
        T: GraphBackend + 'static,
    {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
            backend,
        }
    }

    /// Create or replace a view template.
    pub async fn create_set(&self, template: SetDefinition) -> Result<(), SetsError> {
        let mut templates = self.templates.write().await;
        templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// Update an existing template.
    pub async fn update_set(&self, template: SetDefinition) -> Result<(), SetsError> {
        let mut templates = self.templates.write().await;
        if !templates.contains_key(&template.id) {
            return Err(SetsError::SetNotFound(template.id));
        }
        templates.insert(template.id.clone(), template);
        Ok(())
    }

    /// Get a template by id.
    pub async fn get_set(&self, id: &str) -> Result<Option<SetDefinition>, SetsError> {
        let templates = self.templates.read().await;
        Ok(templates.get(id).cloned())
    }

    /// List all template ids.
    pub async fn list_sets(&self) -> Result<Vec<String>, SetsError> {
        let templates = self.templates.read().await;
        let mut ids = templates.keys().cloned().collect::<Vec<_>>();
        ids.sort();
        Ok(ids)
    }

    /// Bind a template (produce a BoundSet).
    pub async fn bind(&self, bound: BoundSet) -> Result<BoundSet, SetsError> {
        let templates = self.templates.read().await;
        if !templates.contains_key(&bound.set_id) {
            return Err(SetsError::SetNotFound(bound.set_id));
        }
        Ok(bound)
    }

    /// Resolve a bound view on the graph.
    pub async fn resolve(&self, bound: BoundSet) -> Result<ResolvedSet, SetsError> {
        let template = self.load_merged_set(&bound.set_id, &mut Vec::new()).await?;
        let outcome = self
            .resolve_set(template.clone(), &bound, &mut Vec::new())
            .await?;

        let set_kind = Some(set_kind_name(&template.kind).to_string());
        let completeness = if outcome.set.is_empty() {
            Some("empty".to_string())
        } else if outcome.degradations.is_empty() {
            Some("full".to_string())
        } else {
            Some("degraded".to_string())
        };
        let cost_estimate = Some(outcome.set.cost() as u64);
        let subgraph = outcome.set.into_subgraph();

        Ok(ResolvedSet {
            set_id: template.id,
            set_kind,
            nodes: subgraph.nodes,
            edges: subgraph.edges,
            composition_trace: outcome.trace,
            completeness,
            degradations: outcome.degradations,
            cost_estimate,
        })
    }

    #[async_recursion]
    async fn resolve_set(
        &self,
        template: SetDefinition,
        bound: &BoundSet,
        stack: &mut Vec<String>,
    ) -> Result<ResolutionOutcome, SetsError> {
        if stack.iter().any(|id| id == &template.id) {
            return Err(SetsError::SetCycle(template.id));
        }
        stack.push(template.id.clone());

        let mut outcome = ResolutionOutcome::default();
        outcome.trace.push(format!("resolve {}", template.id));
        if let Some(scope) = &bound.resolution_scope {
            outcome
                .trace
                .push(format!("resolution_scope={scope} (informational)"));
        }

        outcome.set = self.seed_set(&template, bound, &mut outcome.trace).await?;
        for filter in &template.filters {
            apply_filter_expression(&mut outcome.set, filter)?;
            outcome.trace.push(format!("filter {filter}"));
        }

        for operation in &template.operations {
            self.apply_operation(operation, bound, &mut outcome, stack)
                .await?;
        }

        let effective_cost_limit = bound
            .parameters
            .get("cost_limit")
            .and_then(|value| value.as_u64())
            .map(|value| value as u32)
            .or(template.cost_limit);
        if let Some(cost_limit) = effective_cost_limit {
            if outcome.set.cost() > cost_limit as usize {
                outcome
                    .degradations
                    .push(format!("cost_limit applied ({cost_limit})"));
                outcome.set.enforce_cost_limit(cost_limit as usize);
                outcome.trace.push(format!("cost_limit {cost_limit}"));
            }
        }

        stack.pop();
        Ok(outcome)
    }

    async fn apply_operation(
        &self,
        operation: &SetOperation,
        bound: &BoundSet,
        outcome: &mut ResolutionOutcome,
        stack: &mut Vec<String>,
    ) -> Result<(), SetsError> {
        match operation {
            SetOperation::Union { set_ids } => {
                for set_id in set_ids {
                    let template = self.load_merged_set(set_id, &mut Vec::new()).await?;
                    let child = self.resolve_set(template, bound, stack).await?;
                    outcome.set.union_with(&child.set);
                    outcome.trace.push(format!("union {set_id}"));
                    outcome.degradations.extend(
                        child
                            .degradations
                            .into_iter()
                            .map(|degradation| format!("child {set_id}: {degradation}")),
                    );
                }
            }
            SetOperation::Intersection { set_ids } => {
                let mut first = outcome.set.clone();
                let mut has_child = false;
                for set_id in set_ids {
                    let template = self.load_merged_set(set_id, &mut Vec::new()).await?;
                    let child = self.resolve_set(template, bound, stack).await?;
                    if !has_child && outcome.set.is_empty() {
                        first = child.set.clone();
                    } else {
                        first.intersect_with(&child.set);
                    }
                    has_child = true;
                    outcome.trace.push(format!("intersection {set_id}"));
                    outcome.degradations.extend(
                        child
                            .degradations
                            .into_iter()
                            .map(|degradation| format!("child {set_id}: {degradation}")),
                    );
                }
                if has_child {
                    outcome.set = first;
                }
            }
            SetOperation::Difference { a, b } => {
                let template_a = self.load_merged_set(a, &mut Vec::new()).await?;
                let template_b = self.load_merged_set(b, &mut Vec::new()).await?;
                let resolved_a = self.resolve_set(template_a, bound, stack).await?;
                let resolved_b = self.resolve_set(template_b, bound, stack).await?;

                let mut difference = resolved_a.set;
                difference.difference_with(&resolved_b.set);
                outcome.set = difference;
                outcome.trace.push(format!("difference {a} minus {b}"));
                outcome.degradations.extend(
                    resolved_b
                        .degradations
                        .into_iter()
                        .map(|degradation| format!("child {b}: {degradation}")),
                );
            }
            SetOperation::Expand {
                relation_type,
                depth,
            } => {
                let subgraph = self
                    .backend
                    .neighborhood(&outcome.set.node_ids(), relation_type.as_deref(), *depth)
                    .await?;
                outcome
                    .set
                    .union_with(&MaterializedSet::from_subgraph(subgraph));
                outcome.trace.push(format!(
                    "expand relation={} depth={}",
                    relation_type.as_deref().unwrap_or("*"),
                    depth
                ));
            }
            SetOperation::FilterNodes { predicate } => {
                apply_node_predicate(&mut outcome.set, predicate)?;
                outcome.trace.push(format!("filter_nodes {predicate}"));
            }
            SetOperation::FilterEdges { predicate } => {
                apply_edge_predicate(&mut outcome.set, predicate)?;
                outcome.trace.push(format!("filter_edges {predicate}"));
            }
            SetOperation::Project { mode } => match outcome.set.project(mode) {
                Ok(()) => outcome.trace.push(format!("project {mode}")),
                Err(error) => {
                    outcome
                        .degradations
                        .push(format!("project degraded: {error}"));
                    outcome.trace.push(format!("project {mode} (degraded)"));
                }
            },
            SetOperation::Slice { limit, order } => {
                outcome.set.slice(*limit as usize, order.as_deref());
                outcome.trace.push(format!(
                    "slice limit={} order={}",
                    limit,
                    order.as_deref().unwrap_or("id_asc")
                ));
            }
        }
        Ok(())
    }

    async fn seed_set(
        &self,
        template: &SetDefinition,
        bound: &BoundSet,
        trace: &mut Vec<String>,
    ) -> Result<MaterializedSet, SetsError> {
        let node_ids = bound_node_ids(bound, &template.selectors);
        if !node_ids.is_empty() {
            trace.push(format!("seed node_ids {:?}", node_ids));
            let subgraph = self.backend.get_subgraph(&node_ids).await?;
            return Ok(MaterializedSet::from_subgraph(subgraph));
        }

        let label = bound_label(bound, &template.selectors);
        let props = bound_props(bound, &template.selectors)?;
        if label.is_some() || !props.is_empty() {
            trace.push(format!(
                "seed selector label={} props={}",
                label.as_deref().unwrap_or("*"),
                props.len()
            ));
            let nodes = self.backend.select_nodes(label.as_deref(), &props).await?;
            let ids = nodes.into_iter().map(|node| node.id).collect::<Vec<_>>();
            if ids.is_empty() {
                return Ok(MaterializedSet::default());
            }
            let subgraph = self.backend.get_subgraph(&ids).await?;
            return Ok(MaterializedSet::from_subgraph(subgraph));
        }

        trace.push("seed empty".to_string());
        Ok(MaterializedSet::default())
    }

    #[async_recursion]
    async fn load_merged_set(
        &self,
        set_id: &str,
        stack: &mut Vec<String>,
    ) -> Result<SetDefinition, SetsError> {
        if stack.iter().any(|id| id == set_id) {
            return Err(SetsError::SetCycle(set_id.to_string()));
        }
        stack.push(set_id.to_string());

        let template = self
            .get_set(set_id)
            .await?
            .ok_or_else(|| SetsError::SetNotFound(set_id.to_string()))?;

        let mut bases = Vec::new();
        for parent_id in &template.extends {
            bases.push(self.load_merged_set(parent_id, stack).await?);
        }

        stack.pop();
        Ok(merge_set_definitions(&template, &bases))
    }
}

fn merge_set_definitions(template: &SetDefinition, bases: &[SetDefinition]) -> SetDefinition {
    let mut selectors = SetSelectors::default();
    let mut filters = Vec::new();
    let mut operations = Vec::new();
    let mut cost_limit = None;
    let mut description = template.description.clone();

    for base in bases {
        selectors.node_ids.extend(base.selectors.node_ids.clone());
        if selectors.label.is_none() {
            selectors.label = base.selectors.label.clone();
        }
        selectors.props.extend(base.selectors.props.clone());
        filters.extend(base.filters.clone());
        operations.extend(base.operations.clone());
        if cost_limit.is_none() {
            cost_limit = base.cost_limit;
        }
        if description.is_empty() {
            description = base.description.clone();
        }
    }

    selectors
        .node_ids
        .extend(template.selectors.node_ids.clone());
    selectors.node_ids.sort_unstable();
    selectors.node_ids.dedup();
    if template.selectors.label.is_some() {
        selectors.label = template.selectors.label.clone();
    }
    selectors.props.extend(template.selectors.props.clone());
    filters.extend(template.filters.clone());
    operations.extend(template.operations.clone());
    if template.cost_limit.is_some() {
        cost_limit = template.cost_limit;
    }

    SetDefinition {
        id: template.id.clone(),
        name: template.name.clone(),
        kind: template.kind.clone(),
        description,
        extends: template.extends.clone(),
        selectors,
        filters,
        operations,
        cost_limit,
    }
}

fn bound_node_ids(bound: &BoundSet, selectors: &SetSelectors) -> Vec<i64> {
    let mut ids = bound
        .anchors
        .get("node_ids")
        .and_then(|value| serde_json::from_value::<Vec<i64>>(value.clone()).ok())
        .unwrap_or_else(|| selectors.node_ids.clone());
    ids.sort_unstable();
    ids.dedup();
    ids
}

fn bound_label(bound: &BoundSet, selectors: &SetSelectors) -> Option<String> {
    bound
        .anchors
        .get("label")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| selectors.label.clone())
}

fn bound_props(
    bound: &BoundSet,
    selectors: &SetSelectors,
) -> Result<HashMap<String, serde_json::Value>, SetsError> {
    if let Some(props) = bound.anchors.get("props") {
        if let Some(object) = props.as_object() {
            return Ok(object
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect());
        }
        return Err(SetsError::Serialization(
            "anchors.props must be a JSON object".to_string(),
        ));
    }
    Ok(selectors.props.clone())
}

fn apply_filter_expression(set: &mut MaterializedSet, predicate: &str) -> Result<(), SetsError> {
    if predicate.starts_with("type:") || predicate.starts_with("rel_type:") {
        apply_edge_predicate(set, predicate)
    } else {
        apply_node_predicate(set, predicate)
    }
}

fn apply_node_predicate(set: &mut MaterializedSet, predicate: &str) -> Result<(), SetsError> {
    if let Some(label) = predicate.strip_prefix("label:") {
        let label = label.to_string();
        set.filter_nodes(|node| node.labels.iter().any(|value| value == &label));
        return Ok(());
    }
    if let Some(value) = predicate.strip_prefix("name_contains:") {
        let needle = value.to_lowercase();
        set.filter_nodes(|node| {
            node.properties
                .get("name")
                .and_then(|value| value.as_str())
                .map(|name| name.to_lowercase().contains(&needle))
                .unwrap_or(false)
        });
        return Ok(());
    }
    if let Some((key, raw_value)) = predicate
        .strip_prefix("property:")
        .and_then(split_once_equals)
    {
        let expected = parse_predicate_value(raw_value);
        let key = key.to_string();
        set.filter_nodes(|node| node.properties.get(&key) == Some(&expected));
        return Ok(());
    }
    if let Some((key, raw_value)) = split_once_equals(predicate) {
        let expected = parse_predicate_value(raw_value);
        let key = key.to_string();
        set.filter_nodes(|node| node.properties.get(&key) == Some(&expected));
        return Ok(());
    }

    Err(SetsError::InvalidPredicate(predicate.to_string()))
}

fn apply_edge_predicate(set: &mut MaterializedSet, predicate: &str) -> Result<(), SetsError> {
    if let Some(rel_type) = predicate
        .strip_prefix("type:")
        .or_else(|| predicate.strip_prefix("rel_type:"))
    {
        let rel_type = rel_type.to_string();
        set.filter_edges(|edge| edge.rel_type == rel_type);
        return Ok(());
    }
    if let Some((key, raw_value)) = predicate
        .strip_prefix("property:")
        .and_then(split_once_equals)
    {
        let expected = parse_predicate_value(raw_value);
        let key = key.to_string();
        set.filter_edges(|edge| edge.properties.get(&key) == Some(&expected));
        return Ok(());
    }

    Err(SetsError::InvalidPredicate(predicate.to_string()))
}

fn split_once_equals(value: &str) -> Option<(&str, &str)> {
    let (left, right) = value.split_once('=')?;
    Some((left.trim(), right.trim()))
}

fn parse_predicate_value(raw: &str) -> serde_json::Value {
    if raw.eq_ignore_ascii_case("true") {
        serde_json::Value::Bool(true)
    } else if raw.eq_ignore_ascii_case("false") {
        serde_json::Value::Bool(false)
    } else if let Ok(int) = raw.parse::<i64>() {
        serde_json::Value::Number(int.into())
    } else if let Ok(float) = raw.parse::<f64>() {
        serde_json::Number::from_f64(float)
            .map(serde_json::Value::Number)
            .unwrap_or_else(|| serde_json::Value::String(raw.to_string()))
    } else {
        serde_json::Value::String(raw.to_string())
    }
}

fn set_kind_name(kind: &SetKind) -> &'static str {
    match kind {
        SetKind::Semantic => "semantic",
        SetKind::Boundary => "boundary",
        SetKind::Projection => "projection",
    }
}

fn node_name(node: &GraphNode) -> String {
    node.properties
        .get("name")
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| node.id.to_string())
}

fn first_label(node: &GraphNode) -> String {
    node.labels
        .first()
        .cloned()
        .unwrap_or_else(|| node.id.to_string())
}

#[cfg(test)]
mod tests {
    use super::{GraphBackend, SetsService};
    use crate::{BoundSet, SetDefinition, SetKind, SetOperation, SetSelectors};
    use async_trait::async_trait;
    use memgraph::{GraphEdge, GraphNode, Subgraph};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[derive(Default)]
    struct FakeBackend {
        nodes: Vec<GraphNode>,
        edges: Vec<GraphEdge>,
    }

    #[async_trait]
    impl GraphBackend for FakeBackend {
        async fn get_subgraph(
            &self,
            node_ids: &[i64],
        ) -> Result<Subgraph, memgraph::MemgraphError> {
            let nodes = self
                .nodes
                .iter()
                .filter(|node| node_ids.contains(&node.id))
                .cloned()
                .collect::<Vec<_>>();
            let edges = self
                .edges
                .iter()
                .filter(|edge| node_ids.contains(&edge.from_id) && node_ids.contains(&edge.to_id))
                .cloned()
                .collect::<Vec<_>>();
            Ok(Subgraph { nodes, edges })
        }

        async fn select_nodes(
            &self,
            label: Option<&str>,
            props: &HashMap<String, serde_json::Value>,
        ) -> Result<Vec<GraphNode>, memgraph::MemgraphError> {
            let nodes = self
                .nodes
                .iter()
                .filter(|node| {
                    let label_ok = label
                        .map(|label| node.labels.iter().any(|value| value == label))
                        .unwrap_or(true);
                    let props_ok = props
                        .iter()
                        .all(|(key, expected)| node.properties.get(key) == Some(expected));
                    label_ok && props_ok
                })
                .cloned()
                .collect();
            Ok(nodes)
        }

        async fn neighborhood(
            &self,
            seed_ids: &[i64],
            relation_type: Option<&str>,
            depth: u32,
        ) -> Result<Subgraph, memgraph::MemgraphError> {
            let mut visited = seed_ids.to_vec();
            let mut frontier = seed_ids.to_vec();
            let mut edges = Vec::new();

            for _ in 0..depth {
                let mut next = Vec::new();
                for edge in &self.edges {
                    if relation_type
                        .map(|value| value == edge.rel_type)
                        .unwrap_or(true)
                    {
                        if frontier.contains(&edge.from_id) && !visited.contains(&edge.to_id) {
                            visited.push(edge.to_id);
                            next.push(edge.to_id);
                            edges.push(edge.clone());
                        } else if frontier.contains(&edge.to_id) && !visited.contains(&edge.from_id)
                        {
                            visited.push(edge.from_id);
                            next.push(edge.from_id);
                            edges.push(edge.clone());
                        } else if frontier.contains(&edge.from_id) || frontier.contains(&edge.to_id)
                        {
                            edges.push(edge.clone());
                        }
                    }
                }
                frontier = next;
            }

            let nodes = self
                .nodes
                .iter()
                .filter(|node| visited.contains(&node.id))
                .cloned()
                .collect::<Vec<_>>();
            Ok(Subgraph { nodes, edges })
        }
    }

    fn node(id: i64, name: &str, labels: &[&str]) -> GraphNode {
        GraphNode {
            id,
            labels: labels.iter().map(|label| label.to_string()).collect(),
            properties: HashMap::from([(
                "name".to_string(),
                serde_json::Value::String(name.to_string()),
            )]),
        }
    }

    fn edge(id: i64, from_id: i64, to_id: i64, rel_type: &str) -> GraphEdge {
        GraphEdge {
            id,
            from_id,
            to_id,
            rel_type: rel_type.to_string(),
            properties: HashMap::new(),
        }
    }

    fn service() -> SetsService {
        SetsService::with_backend(Arc::new(FakeBackend {
            nodes: vec![
                node(1, "Pricing", &["Concept", "Business"]),
                node(2, "Margin", &["Concept", "Business"]),
                node(3, "Forecasting", &["Concept", "Business"]),
                node(4, "BrandIdentity", &["Concept", "Brand"]),
            ],
            edges: vec![
                edge(10, 1, 2, "RELATES_TO"),
                edge(11, 3, 1, "DEPENDS_ON"),
                edge(12, 4, 1, "BRANDS"),
            ],
        }))
    }

    #[tokio::test]
    async fn resolve_supports_label_selectors_and_closed_slice() {
        let service = service();
        service
            .create_set(SetDefinition {
                id: "business".to_string(),
                name: "Business".to_string(),
                kind: SetKind::Semantic,
                description: "business concepts".to_string(),
                extends: vec![],
                selectors: SetSelectors {
                    node_ids: vec![],
                    label: Some("Business".to_string()),
                    props: HashMap::new(),
                },
                filters: vec![],
                operations: vec![SetOperation::Slice {
                    limit: 2,
                    order: Some("name_asc".to_string()),
                }],
                cost_limit: None,
            })
            .await
            .unwrap();

        let resolved = service
            .resolve(BoundSet {
                set_id: "business".to_string(),
                anchors: HashMap::new(),
                parameters: HashMap::new(),
                resolution_scope: Some("playground".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(resolved.nodes.len(), 2);
        assert!(resolved
            .edges
            .iter()
            .all(|edge| resolved.nodes.iter().any(|node| node.id == edge.from_id)));
        assert!(resolved
            .edges
            .iter()
            .all(|edge| resolved.nodes.iter().any(|node| node.id == edge.to_id)));
    }

    #[tokio::test]
    async fn resolve_supports_union_difference_and_expand() {
        let service = service();
        service
            .create_set(SetDefinition {
                id: "core".to_string(),
                name: "Core".to_string(),
                kind: SetKind::Semantic,
                description: "".to_string(),
                extends: vec![],
                selectors: SetSelectors {
                    node_ids: vec![1],
                    label: None,
                    props: HashMap::new(),
                },
                filters: vec![],
                operations: vec![SetOperation::Expand {
                    relation_type: None,
                    depth: 1,
                }],
                cost_limit: None,
            })
            .await
            .unwrap();
        service
            .create_set(SetDefinition {
                id: "brand".to_string(),
                name: "Brand".to_string(),
                kind: SetKind::Boundary,
                description: "".to_string(),
                extends: vec![],
                selectors: SetSelectors {
                    node_ids: vec![4],
                    label: None,
                    props: HashMap::new(),
                },
                filters: vec![],
                operations: vec![],
                cost_limit: None,
            })
            .await
            .unwrap();
        service
            .create_set(SetDefinition {
                id: "shared_base".to_string(),
                name: "SharedBase".to_string(),
                kind: SetKind::Projection,
                description: "".to_string(),
                extends: vec![],
                selectors: SetSelectors {
                    node_ids: vec![2],
                    label: None,
                    props: HashMap::new(),
                },
                filters: vec![],
                operations: vec![SetOperation::Union {
                    set_ids: vec!["core".to_string()],
                }],
                cost_limit: None,
            })
            .await
            .unwrap();
        service
            .create_set(SetDefinition {
                id: "shared".to_string(),
                name: "Shared".to_string(),
                kind: SetKind::Projection,
                description: "".to_string(),
                extends: vec![],
                selectors: SetSelectors::default(),
                filters: vec![],
                operations: vec![SetOperation::Difference {
                    a: "shared_base".to_string(),
                    b: "brand".to_string(),
                }],
                cost_limit: None,
            })
            .await
            .unwrap();

        let resolved = service
            .resolve(BoundSet {
                set_id: "shared".to_string(),
                anchors: HashMap::new(),
                parameters: HashMap::new(),
                resolution_scope: None,
            })
            .await
            .unwrap();

        let node_ids = resolved
            .nodes
            .iter()
            .map(|node| node.id)
            .collect::<Vec<_>>();
        assert!(node_ids.contains(&1));
        assert!(node_ids.contains(&2));
        assert!(node_ids.contains(&3));
        assert!(!node_ids.contains(&4));
    }
}
