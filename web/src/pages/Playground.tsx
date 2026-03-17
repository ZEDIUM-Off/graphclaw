import { useEffect, useMemo, useState } from 'react';
import {
  playgroundCreateNode,
  playgroundCreateEdge,
  playgroundGetGraph,
  playgroundGetView,
  playgroundGetSubgraph,
  playgroundInspectNode,
  playgroundListViews,
  playgroundCreateView,
  playgroundUpdateView,
  playgroundResolve,
  playgroundExport,
  type GraphNodeDto,
  type GraphEdgeDto,
  type ResolvedViewDto,
  type ViewOperationDto,
  type ViewTemplateDto,
} from '@/lib/api';
import PlaygroundCanvas from '@/components/playground/PlaygroundCanvas';
import PlaygroundInspector from '@/components/playground/PlaygroundInspector';

interface CanvasGraphState {
  title: string;
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  emphasizedNodeIds: number[];
  emphasizedEdgeIds: number[];
}

export const EMPTY_GRAPH: CanvasGraphState = {
  title: 'Playground graph',
  nodes: [],
  edges: [],
  emphasizedNodeIds: [],
  emphasizedEdgeIds: [],
};

export function parseCsvIds(value: string): number[] {
  return value
    .split(',')
    .map((item) => Number(item.trim()))
    .filter((item) => !Number.isNaN(item));
}

export function splitList(value: string): string[] {
  return value
    .split(/[\n,]/)
    .map((item) => item.trim())
    .filter(Boolean);
}

export function parseJsonObject(value: string, fieldName: string): Record<string, unknown> {
  if (!value.trim()) {
    return {};
  }

  const parsed: unknown = JSON.parse(value);
  if (parsed === null || typeof parsed !== 'object' || Array.isArray(parsed)) {
    throw new Error(`${fieldName} must be a JSON object`);
  }
  return parsed as Record<string, unknown>;
}

export function parseBoundAnchors(value: string): Record<string, unknown> {
  if (!value.trim()) {
    return {};
  }

  try {
    return parseJsonObject(value, 'Anchors');
  } catch {
    return { node_ids: parseCsvIds(value) };
  }
}

export function parseViewOperation(value: unknown): ViewOperationDto {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    throw new Error('Each operation must be an object');
  }

  const candidate = value as Record<string, unknown>;
  const op = candidate.op;
  if (typeof op !== 'string') {
    throw new Error('Operation is missing "op"');
  }

  switch (op) {
    case 'union':
    case 'intersection': {
      const viewIds = candidate.view_ids;
      if (!Array.isArray(viewIds) || !viewIds.every((item) => typeof item === 'string')) {
        throw new Error(`${op} requires "view_ids" as a string array`);
      }
      return { op, view_ids: viewIds };
    }
    case 'difference': {
      if (typeof candidate.a !== 'string' || typeof candidate.b !== 'string') {
        throw new Error('difference requires "a" and "b"');
      }
      return { op, a: candidate.a, b: candidate.b };
    }
    case 'expand': {
      if (typeof candidate.depth !== 'number') {
        throw new Error('expand requires numeric "depth"');
      }
      if (
        candidate.relation_type !== undefined &&
        typeof candidate.relation_type !== 'string'
      ) {
        throw new Error('expand relation_type must be a string when provided');
      }
      return {
        op,
        depth: candidate.depth,
        relation_type: candidate.relation_type,
      };
    }
    case 'filter_nodes':
    case 'filter_edges': {
      if (typeof candidate.predicate !== 'string') {
        throw new Error(`${op} requires "predicate"`);
      }
      return { op, predicate: candidate.predicate };
    }
    case 'project': {
      if (typeof candidate.mode !== 'string') {
        throw new Error('project requires "mode"');
      }
      return { op, mode: candidate.mode };
    }
    case 'slice': {
      if (typeof candidate.limit !== 'number') {
        throw new Error('slice requires numeric "limit"');
      }
      if (candidate.order !== undefined && typeof candidate.order !== 'string') {
        throw new Error('slice order must be a string when provided');
      }
      return { op, limit: candidate.limit, order: candidate.order };
    }
    default:
      throw new Error(`Unsupported operation: ${op}`);
  }
}

export function parseOperations(value: string): ViewOperationDto[] {
  if (!value.trim()) {
    return [];
  }

  const parsed: unknown = JSON.parse(value);
  if (!Array.isArray(parsed)) {
    throw new Error('Operations must be a JSON array');
  }
  return parsed.map(parseViewOperation);
}

export function upsertNode(nodes: GraphNodeDto[], next: GraphNodeDto): GraphNodeDto[] {
  const remaining = nodes.filter((node) => node.id !== next.id);
  return [...remaining, next].sort((left, right) => left.id - right.id);
}

export function upsertEdge(edges: GraphEdgeDto[], next: GraphEdgeDto): GraphEdgeDto[] {
  const remaining = edges.filter((edge) => edge.id !== next.id);
  return [...remaining, next].sort((left, right) => left.id - right.id);
}

export function mergeNodeIds(current: number[], incoming: number[]): number[] {
  return [...new Set([...current, ...incoming])].sort((left, right) => left - right);
}

export function formatSelectionAnchors(nodeIds: number[]): string {
  return JSON.stringify({ node_ids: nodeIds }, null, 2);
}

export function formatTemplate(template: ViewTemplateDto) {
  return {
    extendsText: (template.extends ?? []).join(', '),
    nodeIdsText: (template.selectors?.node_ids ?? []).join(', '),
    selectorLabel: template.selectors?.label ?? '',
    selectorPropsText: JSON.stringify(template.selectors?.props ?? {}, null, 2),
    filtersText: (template.filters ?? []).join('\n'),
    operationsText: JSON.stringify(template.operations ?? [], null, 2),
    costLimitText:
      typeof template.cost_limit === 'number' ? String(template.cost_limit) : '',
  };
}

export default function Playground() {
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [canvasGraph, setCanvasGraph] = useState<CanvasGraphState>(EMPTY_GRAPH);
  const [isGraphLoading, setIsGraphLoading] = useState(true);
  const [selectedNodeIds, setSelectedNodeIds] = useState<number[]>([]);
  const [primarySelectedNodeId, setPrimarySelectedNodeId] = useState<number | null>(null);
  const [selectedEdgeId, setSelectedEdgeId] = useState<number | null>(null);
  const [inspectedNode, setInspectedNode] = useState<GraphNodeDto | null>(null);
  const [selectionModifierActive, setSelectionModifierActive] = useState(false);

  // Graph zone state
  const [nodeLabels, setNodeLabels] = useState('Concept');
  const [nodeName, setNodeName] = useState('');
  const [createdNodes, setCreatedNodes] = useState<GraphNodeDto[]>([]);
  const [fromId, setFromId] = useState('');
  const [toId, setToId] = useState('');
  const [relType, setRelType] = useState('RELATES_TO');
  const [subgraphIds, setSubgraphIds] = useState('');
  const [subgraphResult, setSubgraphResult] = useState<{ nodes: GraphNodeDto[]; edges: GraphEdgeDto[] } | null>(null);

  // View zone state
  const [viewId, setViewId] = useState('');
  const [viewName, setViewName] = useState('');
  const [viewKind, setViewKind] = useState<'semantic' | 'boundary' | 'projection'>('semantic');
  const [viewDescription, setViewDescription] = useState('');
  const [viewExtends, setViewExtends] = useState('');
  const [viewNodeIds, setViewNodeIds] = useState('');
  const [viewSelectorLabel, setViewSelectorLabel] = useState('');
  const [viewSelectorProps, setViewSelectorProps] = useState('{}');
  const [viewFilters, setViewFilters] = useState('');
  const [viewOperations, setViewOperations] = useState('[]');
  const [viewCostLimit, setViewCostLimit] = useState('');
  const [templateList, setTemplateList] = useState<string[]>([]);
  const [loadedTemplateId, setLoadedTemplateId] = useState<string | null>(null);

  // Resolved zone state
  const [resolveTemplateId, setResolveTemplateId] = useState('');
  const [resolveAnchors, setResolveAnchors] = useState('');
  const [resolvedView, setResolvedView] = useState<ResolvedViewDto | null>(null);

  // Export zone state
  const [exportFormat, setExportFormat] = useState<'llm_compact' | 'llm_explained'>('llm_compact');
  const [exportPurpose, setExportPurpose] = useState('Shared business logic across the selected graph.');
  const [exportRole, setExportRole] = useState('Use this view as structured context for business reasoning.');
  const [exportConstraints, setExportConstraints] = useState('business-only');
  const [exportUsageHint, setExportUsageHint] = useState('Use for reasoning, not as a raw database dump.');
  const [exportIncluded, setExportIncluded] = useState('');
  const [exportExcluded, setExportExcluded] = useState('');
  const [exportResult, setExportResult] = useState<string | null>(null);

  const selectedNode = useMemo(() => {
    if (inspectedNode && inspectedNode.id === primarySelectedNodeId) {
      return inspectedNode;
    }
    return canvasGraph.nodes.find((node) => node.id === primarySelectedNodeId) ?? null;
  }, [canvasGraph.nodes, inspectedNode, primarySelectedNodeId]);

  const selectedEdge = useMemo(
    () => canvasGraph.edges.find((edge) => edge.id === selectedEdgeId) ?? null,
    [canvasGraph.edges, selectedEdgeId],
  );

  useEffect(() => {
    void (async () => {
      setIsGraphLoading(true);
      const [viewsResult, graphResult] = await Promise.allSettled([
        playgroundListViews(),
        playgroundGetGraph(),
      ]);

      if (viewsResult.status === 'fulfilled') {
        setTemplateList(viewsResult.value);
      } else {
        setError(
          viewsResult.reason instanceof Error
            ? viewsResult.reason.message
            : 'Failed to list views',
        );
      }

      if (graphResult.status === 'fulfilled') {
        const snapshot = graphResult.value;
        setCanvasGraph({
          title: snapshot.meta.truncated
            ? 'Playground graph snapshot (bounded)'
            : 'Playground graph',
          nodes: snapshot.nodes,
          edges: snapshot.edges,
          emphasizedNodeIds: [],
          emphasizedEdgeIds: [],
        });
      } else {
        setError(
          graphResult.reason instanceof Error
            ? graphResult.reason.message
            : 'Failed to load graph snapshot',
        );
      }

      setIsGraphLoading(false);
    })();
  }, []);

  useEffect(() => {
    const handleKeyState = (event: KeyboardEvent) => {
      setSelectionModifierActive(event.ctrlKey || event.metaKey);
    };

    const handleWindowBlur = () => {
      setSelectionModifierActive(false);
    };

    window.addEventListener('keydown', handleKeyState);
    window.addEventListener('keyup', handleKeyState);
    window.addEventListener('blur', handleWindowBlur);
    return () => {
      window.removeEventListener('keydown', handleKeyState);
      window.removeEventListener('keyup', handleKeyState);
      window.removeEventListener('blur', handleWindowBlur);
    };
  }, []);

  const resetSelection = () => {
    setSelectedNodeIds([]);
    setPrimarySelectedNodeId(null);
    setSelectedEdgeId(null);
    setInspectedNode(null);
  };

  const inspectSelectedNode = async (nodeId: number | null) => {
    setPrimarySelectedNodeId(nodeId);
    if (nodeId === null) {
      setInspectedNode(null);
      return;
    }

    try {
      const node = await playgroundInspectNode(nodeId);
      setInspectedNode(node);
    } catch {
      setInspectedNode(canvasGraph.nodes.find((node) => node.id === nodeId) ?? null);
    }
  };

  const handleCreateNode = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setNotice(null);
    try {
      const labels = nodeLabels.split(',').map((s) => s.trim()).filter(Boolean);
      const properties: Record<string, unknown> = nodeName ? { name: nodeName } : {};
      const node = await playgroundCreateNode(labels, properties);
      setCreatedNodes((prev) => [...prev, node]);
      setCanvasGraph((prev) => ({
        ...prev,
        title: 'Manual playground seed',
        nodes: upsertNode(prev.nodes, node),
      }));
      setNotice(`Created node ${node.id}.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create node');
    }
  };

  const handleCreateEdge = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setNotice(null);
    try {
      const edge = await playgroundCreateEdge(Number(fromId), Number(toId), relType, {});
      setCanvasGraph((prev) => ({
        ...prev,
        title: 'Manual playground seed',
        edges: upsertEdge(prev.edges, edge),
      }));
      setNotice(`Created edge ${edge.id}.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create edge');
    }
  };

  const handleGetSubgraph = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setNotice(null);
    try {
      const ids = parseCsvIds(subgraphIds);
      const sg = await playgroundGetSubgraph(ids);
      setSubgraphResult(sg);
      setCanvasGraph({
        title: 'Fetched subgraph',
        nodes: sg.nodes,
        edges: sg.edges,
        emphasizedNodeIds: [],
        emphasizedEdgeIds: [],
      });
      resetSelection();
      setNotice(`Loaded subgraph with ${sg.nodes.length} nodes.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to get subgraph');
    }
  };

  const handleCreateView = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setNotice(null);
    try {
      const node_ids = parseCsvIds(viewNodeIds);
      const selectorProps = parseJsonObject(viewSelectorProps, 'Selector props');
      const operations = parseOperations(viewOperations);
      const template: ViewTemplateDto = {
        id: viewId,
        name: viewName,
        kind: viewKind,
        description: viewDescription,
        extends: splitList(viewExtends),
        selectors: {
          node_ids,
          label: viewSelectorLabel || undefined,
          props: selectorProps,
        },
        filters: splitList(viewFilters),
        operations,
        cost_limit: viewCostLimit.trim() ? Number(viewCostLimit) : undefined,
      };
      const isExistingTemplate = templateList.includes(viewId);
      if (isExistingTemplate) {
        await playgroundUpdateView(viewId, template);
      } else {
        await playgroundCreateView(template);
      }
      setTemplateList((prev) =>
        prev.includes(viewId) ? [...prev].sort() : [...prev, viewId].sort(),
      );
      setLoadedTemplateId(viewId);
      setNotice(`${isExistingTemplate ? 'Updated' : 'Saved'} view template ${viewId}.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save view');
    }
  };

  const loadViewList = async () => {
    setError(null);
    try {
      const ids = await playgroundListViews();
      setTemplateList(ids);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to list views');
    }
  };

  const handleLoadTemplate = async (templateId: string) => {
    setError(null);
    setNotice(null);
    try {
      const template = await playgroundGetView(templateId);
      const formatted = formatTemplate(template);
      setViewId(template.id);
      setViewName(template.name);
      setViewKind(template.kind as 'semantic' | 'boundary' | 'projection');
      setViewDescription(template.description);
      setViewExtends(formatted.extendsText);
      setViewNodeIds(formatted.nodeIdsText);
      setViewSelectorLabel(formatted.selectorLabel);
      setViewSelectorProps(formatted.selectorPropsText);
      setViewFilters(formatted.filtersText);
      setViewOperations(formatted.operationsText);
      setViewCostLimit(formatted.costLimitText);
      setResolveTemplateId(template.id);
      setLoadedTemplateId(template.id);
      setNotice(`Loaded template ${template.id} into the editor.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load view');
    }
  };

  const handleResolve = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setNotice(null);
    try {
      const anchors = parseBoundAnchors(resolveAnchors);
      const r = await playgroundResolve(resolveTemplateId, anchors);
      setResolvedView(r);
      setCanvasGraph({
        title: `Resolved view: ${r.view_id}`,
        nodes: r.nodes,
        edges: r.edges,
        emphasizedNodeIds: r.nodes.map((node) => node.id),
        emphasizedEdgeIds: r.edges.map((edge) => edge.id),
      });
      resetSelection();
      setNotice(`Resolved ${r.view_id} with ${r.nodes.length} nodes.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to resolve view');
    }
  };

  const handleExport = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    if (!resolvedView) {
      setError('Resolve a view first');
      return;
    }
    try {
      const result = await playgroundExport(resolvedView, exportFormat, {
        purpose: exportPurpose,
        constraints: splitList(exportConstraints),
        usage_hint: exportUsageHint,
        role: exportRole,
        included: splitList(exportIncluded),
        excluded: splitList(exportExcluded),
      });
      setExportResult(result.text);
      setNotice(`Generated ${exportFormat} export.`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to export');
    }
  };

  const handleNodeClick = async (nodeId: number) => {
    setSelectedEdgeId(null);
    if (selectionModifierActive) {
      const isSelected = selectedNodeIds.includes(nodeId);
      const nextSelectedNodeIds = isSelected
        ? selectedNodeIds.filter((id) => id !== nodeId)
        : mergeNodeIds(selectedNodeIds, [nodeId]);
      setSelectedNodeIds(nextSelectedNodeIds);
      const nextPrimaryNodeId = isSelected
        ? (nextSelectedNodeIds.length > 0
            ? nextSelectedNodeIds[nextSelectedNodeIds.length - 1] ?? null
            : null)
        : nodeId;
      await inspectSelectedNode(nextPrimaryNodeId);
      return;
    }

    setSelectedNodeIds([nodeId]);
    await inspectSelectedNode(nodeId);
  };

  const handleEdgeClick = (edgeId: number) => {
    setSelectedNodeIds([]);
    setPrimarySelectedNodeId(null);
    setSelectedEdgeId(edgeId);
    setInspectedNode(null);
  };

  const handleUseSelectionInViewEditor = () => {
    setViewNodeIds(selectedNodeIds.join(', '));
    setNotice(`Injected ${selectedNodeIds.length} selected node(s) into the view editor.`);
  };

  const handleUseSelectionAsResolveAnchors = () => {
    setResolveAnchors(formatSelectionAnchors(selectedNodeIds));
    setNotice(`Injected ${selectedNodeIds.length} selected node(s) into resolve anchors.`);
  };

  const handleMergeSelectionIntoLoadedView = () => {
    if (!loadedTemplateId || loadedTemplateId !== viewId) {
      setError('Load a template into the editor before merging the current selection.');
      return;
    }

    const mergedNodeIds = mergeNodeIds(parseCsvIds(viewNodeIds), selectedNodeIds);
    setViewNodeIds(mergedNodeIds.join(', '));
    setNotice(`Merged ${selectedNodeIds.length} selected node(s) into ${loadedTemplateId}.`);
  };

  return (
    <div className="p-6">
      <div className="max-w-[1600px] mx-auto space-y-6">
        <div>
          <h1 className="text-2xl font-bold text-white mb-2">GraphClaw Playground</h1>
          <p className="text-gray-400">
            Compose and materialise views, resolve them on the current graph, and inspect the
            result in a live Sigma canvas before exporting LLM-ready context.
          </p>
        </div>

        {error && (
          <div className="p-3 rounded-xl bg-red-900/30 border border-red-700 text-red-300 text-sm">
            {error}
          </div>
        )}
        {notice && (
          <div className="p-3 rounded-xl bg-blue-900/20 border border-blue-700 text-blue-200 text-sm">
            {notice}
          </div>
        )}

        <div className="grid gap-6 xl:grid-cols-[26rem_minmax(0,1fr)]">
          <div className="space-y-6">
            <section className="bg-gray-900 rounded-2xl p-4 border border-gray-800">
              <h2 className="text-lg font-semibold text-white mb-3">1. Seed graph</h2>
              <div className="space-y-5">
                <form onSubmit={handleCreateNode} className="space-y-3">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Labels</label>
                    <input
                      type="text"
                      value={nodeLabels}
                      onChange={(e) => setNodeLabels(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="Concept,Business"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Name</label>
                    <input
                      type="text"
                      value={nodeName}
                      onChange={(e) => setNodeName(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="Pricing"
                    />
                  </div>
                  <button
                    type="submit"
                    className="px-4 py-2 bg-blue-600 text-white rounded text-sm font-medium"
                  >
                    Create node
                  </button>
                </form>

                <form onSubmit={handleCreateEdge} className="grid gap-3 md:grid-cols-3">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">From node id</label>
                    <input
                      type="text"
                      value={fromId}
                      onChange={(e) => setFromId(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">To node id</label>
                    <input
                      type="text"
                      value={toId}
                      onChange={(e) => setToId(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Relation type</label>
                    <input
                      type="text"
                      value={relType}
                      onChange={(e) => setRelType(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    />
                  </div>
                  <div className="md:col-span-3">
                    <button
                      type="submit"
                      className="px-4 py-2 bg-blue-600 text-white rounded text-sm font-medium"
                    >
                      Create edge
                    </button>
                  </div>
                </form>

                <form onSubmit={handleGetSubgraph} className="space-y-3">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">
                      Fetch subgraph from node ids
                    </label>
                    <input
                      type="text"
                      value={subgraphIds}
                      onChange={(e) => setSubgraphIds(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="1, 2, 3"
                    />
                  </div>
                  <button
                    type="submit"
                    className="px-4 py-2 bg-gray-700 text-white rounded text-sm font-medium"
                  >
                    Show subgraph
                  </button>
                </form>

                {createdNodes.length > 0 && (
                  <p className="text-xs text-gray-400">
                    Created nodes: {createdNodes.map((node) => node.id).join(', ')}
                  </p>
                )}
              </div>
            </section>

            <section className="bg-gray-900 rounded-2xl p-4 border border-gray-800">
              <div className="flex items-center justify-between mb-3">
                <h2 className="text-lg font-semibold text-white">2. View editor</h2>
                <button
                  type="button"
                  onClick={() => {
                    setLoadedTemplateId(null);
                    setViewId('');
                    setViewName('');
                    setViewKind('semantic');
                    setViewDescription('');
                    setViewExtends('');
                    setViewNodeIds('');
                    setViewSelectorLabel('');
                    setViewSelectorProps('{}');
                    setViewFilters('');
                    setViewOperations('[]');
                    setViewCostLimit('');
                  }}
                  className="text-xs text-gray-400 hover:text-white"
                >
                  Clear
                </button>
              </div>
              <form onSubmit={handleCreateView} className="space-y-3">
                <div className="grid gap-3 md:grid-cols-2">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Id</label>
                    <input
                      type="text"
                      value={viewId}
                      onChange={(e) => setViewId(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="view_business"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Kind</label>
                    <select
                      value={viewKind}
                      onChange={(e) =>
                        setViewKind(e.target.value as 'semantic' | 'boundary' | 'projection')
                      }
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    >
                      <option value="semantic">semantic</option>
                      <option value="boundary">boundary</option>
                      <option value="projection">projection</option>
                    </select>
                  </div>
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Name</label>
                  <input
                    type="text"
                    value={viewName}
                    onChange={(e) => setViewName(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    placeholder="Business logic shared"
                  />
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Description</label>
                  <textarea
                    value={viewDescription}
                    onChange={(e) => setViewDescription(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm min-h-20"
                  />
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Extends</label>
                  <input
                    type="text"
                    value={viewExtends}
                    onChange={(e) => setViewExtends(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    placeholder="view_highfinity_core, view_mc_studio_core"
                  />
                </div>

                <div className="grid gap-3 md:grid-cols-2">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Selector node ids</label>
                    <input
                      type="text"
                      value={viewNodeIds}
                      onChange={(e) => setViewNodeIds(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="1, 2, 3"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Selector label</label>
                    <input
                      type="text"
                      value={viewSelectorLabel}
                      onChange={(e) => setViewSelectorLabel(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="Business"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Selector props (JSON)</label>
                  <textarea
                    value={viewSelectorProps}
                    onChange={(e) => setViewSelectorProps(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-xs min-h-24 font-mono"
                  />
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Filters</label>
                  <textarea
                    value={viewFilters}
                    onChange={(e) => setViewFilters(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-xs min-h-20 font-mono"
                    placeholder={'label:Business\nproperty:workspace=HIGHFINITY'}
                  />
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Operations (JSON)</label>
                  <textarea
                    value={viewOperations}
                    onChange={(e) => setViewOperations(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-xs min-h-32 font-mono"
                    placeholder={'[\n  { "op": "expand", "relation_type": "RELATES_TO", "depth": 1 }\n]'}
                  />
                </div>

                <div>
                  <label className="block text-gray-400 text-sm mb-1">Cost limit</label>
                  <input
                    type="text"
                    value={viewCostLimit}
                    onChange={(e) => setViewCostLimit(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    placeholder="12"
                  />
                </div>

                <button
                  type="submit"
                  className="px-4 py-2 bg-blue-600 text-white rounded text-sm font-medium"
                >
                  Save view template
                </button>
              </form>
            </section>

            <section className="bg-gray-900 rounded-2xl p-4 border border-gray-800">
              <div className="flex items-center justify-between mb-3">
                <h2 className="text-lg font-semibold text-white">3. Templates</h2>
                <button
                  type="button"
                  onClick={loadViewList}
                  className="px-3 py-1.5 bg-gray-700 text-white rounded text-sm"
                >
                  Refresh
                </button>
              </div>
              <div className="space-y-2 max-h-56 overflow-auto">
                {templateList.map((id) => (
                  <button
                    key={id}
                    type="button"
                    onClick={() => {
                      void handleLoadTemplate(id);
                    }}
                    className="w-full text-left px-3 py-2 rounded-lg bg-gray-800 hover:bg-gray-700 text-gray-200 text-sm"
                  >
                    {id}
                  </button>
                ))}
                {templateList.length === 0 && (
                  <div className="text-sm text-gray-500">No templates yet.</div>
                )}
              </div>
            </section>

            <section className="bg-gray-900 rounded-2xl p-4 border border-gray-800">
              <h2 className="text-lg font-semibold text-white mb-3">4. Resolve & export</h2>
              <div className="space-y-5">
                <form onSubmit={handleResolve} className="space-y-3">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Template id</label>
                    <input
                      type="text"
                      value={resolveTemplateId}
                      onChange={(e) => setResolveTemplateId(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      placeholder="view_business"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">
                      Anchors (JSON or node ids)
                    </label>
                    <textarea
                      value={resolveAnchors}
                      onChange={(e) => setResolveAnchors(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-xs min-h-20 font-mono"
                      placeholder='{"node_ids":[1,2],"label":"Business"}'
                    />
                  </div>
                  <button
                    type="submit"
                    className="px-4 py-2 bg-blue-600 text-white rounded text-sm font-medium"
                  >
                    Resolve view
                  </button>
                </form>

                <form onSubmit={handleExport} className="space-y-3">
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Format</label>
                    <select
                      value={exportFormat}
                      onChange={(e) =>
                        setExportFormat(e.target.value as 'llm_compact' | 'llm_explained')
                      }
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    >
                      <option value="llm_compact">llm_compact</option>
                      <option value="llm_explained">llm_explained</option>
                    </select>
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Purpose</label>
                    <input
                      type="text"
                      value={exportPurpose}
                      onChange={(e) => setExportPurpose(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                    />
                  </div>
                  <div>
                    <label className="block text-gray-400 text-sm mb-1">Role / explanation</label>
                    <textarea
                      value={exportRole}
                      onChange={(e) => setExportRole(e.target.value)}
                      className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm min-h-20"
                    />
                  </div>
                  <div className="grid gap-3 md:grid-cols-2">
                    <div>
                      <label className="block text-gray-400 text-sm mb-1">Constraints</label>
                      <input
                        type="text"
                        value={exportConstraints}
                        onChange={(e) => setExportConstraints(e.target.value)}
                        className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                        placeholder="business-only, no brand nodes"
                      />
                    </div>
                    <div>
                      <label className="block text-gray-400 text-sm mb-1">Usage hint</label>
                      <input
                        type="text"
                        value={exportUsageHint}
                        onChange={(e) => setExportUsageHint(e.target.value)}
                        className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-sm"
                      />
                    </div>
                  </div>
                  <div className="grid gap-3 md:grid-cols-2">
                    <div>
                      <label className="block text-gray-400 text-sm mb-1">Included concepts</label>
                      <textarea
                        value={exportIncluded}
                        onChange={(e) => setExportIncluded(e.target.value)}
                        className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-xs min-h-20 font-mono"
                        placeholder="Pricing, Margin"
                      />
                    </div>
                    <div>
                      <label className="block text-gray-400 text-sm mb-1">Excluded concepts</label>
                      <textarea
                        value={exportExcluded}
                        onChange={(e) => setExportExcluded(e.target.value)}
                        className="w-full px-3 py-2 bg-gray-800 border border-gray-700 rounded text-white text-xs min-h-20 font-mono"
                        placeholder="BrandIdentity"
                      />
                    </div>
                  </div>
                  <button
                    type="submit"
                    disabled={!resolvedView}
                    className="px-4 py-2 bg-blue-600 text-white rounded text-sm font-medium disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    Export resolved view
                  </button>
                </form>

                {exportResult && (
                  <button
                    type="button"
                    onClick={() => {
                      void navigator.clipboard.writeText(exportResult);
                    }}
                    className="px-3 py-1.5 bg-gray-700 text-white rounded text-sm"
                  >
                    Copy latest export
                  </button>
                )}
              </div>
            </section>
          </div>

          <div className="space-y-6">
            <PlaygroundCanvas
              title={canvasGraph.title}
              nodes={canvasGraph.nodes}
              edges={canvasGraph.edges}
              isLoading={isGraphLoading}
              emphasizedNodeIds={canvasGraph.emphasizedNodeIds}
              emphasizedEdgeIds={canvasGraph.emphasizedEdgeIds}
              selectedNodeIds={selectedNodeIds}
              primarySelectedNodeId={primarySelectedNodeId}
              selectedEdgeId={selectedEdgeId}
              onNodeClick={(nodeId) => {
                void handleNodeClick(nodeId);
              }}
              onEdgeClick={handleEdgeClick}
            />

            <PlaygroundInspector
              selectedNode={selectedNode}
              selectedNodeIds={selectedNodeIds}
              selectedEdge={selectedEdge}
              resolvedView={resolvedView}
              exportText={exportResult}
              canMergeSelectionIntoView={loadedTemplateId !== null && loadedTemplateId === viewId}
              onUseSelectionInViewEditor={handleUseSelectionInViewEditor}
              onUseSelectionAsResolveAnchors={handleUseSelectionAsResolveAnchors}
              onMergeSelectionIntoLoadedView={handleMergeSelectionIntoLoadedView}
            />

            {subgraphResult && (
              <section className="bg-gray-900 rounded-2xl p-4 border border-gray-800">
                <h2 className="text-lg font-semibold text-white mb-3">Latest fetched subgraph</h2>
                <pre className="bg-gray-950 border border-gray-800 rounded-lg p-3 text-xs text-gray-300 overflow-auto max-h-64">
                  {JSON.stringify(subgraphResult, null, 2)}
                </pre>
              </section>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
