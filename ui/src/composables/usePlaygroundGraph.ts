import { computed, ref } from 'vue';
import {
  createEdge,
  createNode,
  createSetFromSelection,
  getSets,
  inspectGraphNode,
  resolveSet,
  type GraphEdgeDto,
  type GraphNodeDto,
  type ResolvedSetDto,
  type SetDefinitionDto,
} from '@/lib/playground';

const DEFAULT_NODE_LIMIT = 200;
const DEFAULT_EDGE_LIMIT = 400;
const ROOT_SET_ID = 'root';

function normalizePositiveInteger(value: string, fallback: number): number {
  const parsed = Number(value.trim());
  if (!Number.isFinite(parsed) || parsed <= 0) {
    return fallback;
  }
  return Math.max(1, Math.floor(parsed));
}

function parseCsvList(value: string): string[] {
  return value
    .split(',')
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function propertyBagFromText(value: string): Record<string, unknown> {
  if (!value.trim()) {
    return {};
  }

  try {
    const parsed = JSON.parse(value) as unknown;
    if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
      return parsed as Record<string, unknown>;
    }
  } catch {
    // Fall through to a plain string payload.
  }

  return { value };
}

export function usePlaygroundGraph() {
  const sets = ref<SetDefinitionDto[]>([]);
  const resolvedSet = ref<ResolvedSetDto | null>(null);
  const selectedNodeIds = ref<number[]>([]);
  const selectedEdgeId = ref<number | null>(null);
  const inspectedNode = ref<GraphNodeDto | null>(null);
  const error = ref<string | null>(null);
  const isLoading = ref(true);
  const isRefreshing = ref(false);
  const activeTab = ref<'graph' | 'data'>('graph');
  const activeModule = ref<'overview' | 'search' | 'layout' | 'appearance'>('overview');
  const activeSetId = ref(ROOT_SET_ID);
  const lastLoadedAt = ref<Date | null>(null);
  const nodeLimitInput = ref(String(DEFAULT_NODE_LIMIT));
  const edgeLimitInput = ref(String(DEFAULT_EDGE_LIMIT));
  const searchQuery = ref('');
  const layoutMode = ref<'grid' | 'circlepack' | 'radial'>('circlepack');
  const appearanceMode = ref<'labels' | 'sets' | 'density'>('labels');

  const setDraftName = ref('');
  const setDraftDescription = ref('');
  const setDraftKind = ref<'boundary' | 'semantic' | 'projection'>('boundary');

  const nodeDraftLabels = ref('Concept');
  const nodeDraftProperties = ref('{"name": ""}');
  const edgeDraftFromId = ref('');
  const edgeDraftToId = ref('');
  const edgeDraftType = ref('RELATES_TO');
  const edgeDraftProperties = ref('{}');

  const activeNodeLimit = computed(() =>
    normalizePositiveInteger(nodeLimitInput.value, DEFAULT_NODE_LIMIT),
  );
  const activeEdgeLimit = computed(() =>
    normalizePositiveInteger(edgeLimitInput.value, DEFAULT_EDGE_LIMIT),
  );
  const activeSet = computed(
    () => sets.value.find((entry) => entry.id === activeSetId.value) ?? null,
  );
  const selectedNode = computed(() => {
    if (inspectedNode.value && selectedNodeIds.value.includes(inspectedNode.value.id)) {
      return inspectedNode.value;
    }
    const lastId = selectedNodeIds.value.at(-1);
    return resolvedSet.value?.nodes.find((node) => node.id === lastId) ?? null;
  });
  const selectedEdge = computed(
    () => resolvedSet.value?.edges.find((edge) => edge.id === selectedEdgeId.value) ?? null,
  );
  const selectedNodes = computed(() =>
    resolvedSet.value?.nodes.filter((node) => selectedNodeIds.value.includes(node.id)) ?? [],
  );
  const filteredNodes = computed(() => {
    const query = searchQuery.value.trim().toLowerCase();
    const nodes = resolvedSet.value?.nodes ?? [];
    if (!query) {
      return nodes;
    }
    return nodes.filter((node) => {
      const name = String(node.properties.name ?? '').toLowerCase();
      return (
        name.includes(query) ||
        node.labels.some((label) => label.toLowerCase().includes(query)) ||
        String(node.id).includes(query)
      );
    });
  });
  const filteredEdges = computed(() => {
    const query = searchQuery.value.trim().toLowerCase();
    const edges = resolvedSet.value?.edges ?? [];
    if (!query) {
      return edges;
    }
    return edges.filter(
      (edge) =>
        edge.rel_type.toLowerCase().includes(query) ||
        String(edge.from_id).includes(query) ||
        String(edge.to_id).includes(query),
    );
  });
  const filteredSets = computed(() => {
    const query = searchQuery.value.trim().toLowerCase();
    if (!query) {
      return sets.value;
    }
    return sets.value.filter(
      (entry) =>
        entry.name.toLowerCase().includes(query) ||
        entry.id.toLowerCase().includes(query) ||
        entry.kind.toLowerCase().includes(query),
    );
  });
  const graphTitle = computed(() => {
    if (!resolvedSet.value) {
      return 'GraphClaw playground';
    }
    return `${activeSet.value?.name ?? resolvedSet.value.set_id} graph map`;
  });

  async function refreshSets() {
    sets.value = await getSets();
  }

  async function activateSet(setId: string) {
    error.value = null;
    isRefreshing.value = true;

    try {
      resolvedSet.value = await resolveSet({
        setId,
        nodeLimit: setId === ROOT_SET_ID ? activeNodeLimit.value : undefined,
        edgeLimit: setId === ROOT_SET_ID ? activeEdgeLimit.value : undefined,
      });
      activeSetId.value = setId;
      selectedNodeIds.value = [];
      selectedEdgeId.value = null;
      inspectedNode.value = null;
      lastLoadedAt.value = new Date();
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to resolve set';
    } finally {
      isLoading.value = false;
      isRefreshing.value = false;
    }
  }

  async function initialize() {
    await refreshSets();
    await activateSet(ROOT_SET_ID);
  }

  async function refresh() {
    await refreshSets();
    await activateSet(activeSetId.value);
  }

  async function applyRootLimits() {
    if (activeSetId.value !== ROOT_SET_ID) {
      await activateSet(ROOT_SET_ID);
      return;
    }
    await activateSet(ROOT_SET_ID);
  }

  async function toggleNode(nodeId: number) {
    selectedEdgeId.value = null;
    inspectedNode.value = null;
    if (selectedNodeIds.value.includes(nodeId)) {
      selectedNodeIds.value = selectedNodeIds.value.filter((id) => id !== nodeId);
      return;
    }
    selectedNodeIds.value = [...selectedNodeIds.value, nodeId];
    try {
      inspectedNode.value = await inspectGraphNode(nodeId);
    } catch {
      inspectedNode.value = resolvedSet.value?.nodes.find((node) => node.id === nodeId) ?? null;
    }
  }

  function selectEdge(edgeId: number) {
    selectedNodeIds.value = [];
    inspectedNode.value = null;
    selectedEdgeId.value = edgeId;
  }

  function clearSelection() {
    selectedNodeIds.value = [];
    selectedEdgeId.value = null;
    inspectedNode.value = null;
  }

  async function activateSetFromTable(setId: string) {
    activeTab.value = 'graph';
    await activateSet(setId);
  }

  async function createSelectionSet() {
    if (selectedNodeIds.value.length === 0) {
      error.value = 'Select at least one node before creating a set.';
      return;
    }

    const name = setDraftName.value.trim() || `Selection ${selectedNodeIds.value.length}`;
    await createSetFromSelection({
      name,
      description:
        setDraftDescription.value.trim() ||
        `Set created from ${selectedNodeIds.value.length} selected nodes.`,
      kind: setDraftKind.value,
      node_ids: [...selectedNodeIds.value],
      cost_limit: null,
    });

    setDraftName.value = '';
    setDraftDescription.value = '';
    await refreshSets();
  }

  async function submitNodeDraft() {
    await createNode({
      labels: parseCsvList(nodeDraftLabels.value),
      properties: propertyBagFromText(nodeDraftProperties.value),
    });
    await refresh();
  }

  async function submitEdgeDraft() {
    await createEdge({
      from_id: normalizePositiveInteger(edgeDraftFromId.value, 0),
      to_id: normalizePositiveInteger(edgeDraftToId.value, 0),
      rel_type: edgeDraftType.value.trim() || 'RELATES_TO',
      properties: propertyBagFromText(edgeDraftProperties.value),
    });
    await refresh();
  }

  return {
    sets,
    resolvedSet,
    selectedNodeIds,
    selectedNode,
    selectedNodes,
    selectedEdge,
    error,
    isLoading,
    isRefreshing,
    activeTab,
    activeModule,
    activeSetId,
    activeSet,
    lastLoadedAt,
    nodeLimitInput,
    edgeLimitInput,
    activeNodeLimit,
    activeEdgeLimit,
    searchQuery,
    layoutMode,
    appearanceMode,
    filteredNodes,
    filteredEdges,
    filteredSets,
    graphTitle,
    setDraftName,
    setDraftDescription,
    setDraftKind,
    nodeDraftLabels,
    nodeDraftProperties,
    edgeDraftFromId,
    edgeDraftToId,
    edgeDraftType,
    edgeDraftProperties,
    initialize,
    refresh,
    applyRootLimits,
    toggleNode,
    selectEdge,
    clearSelection,
    activateSetFromTable,
    createSelectionSet,
    submitNodeDraft,
    submitEdgeDraft,
  };
}
