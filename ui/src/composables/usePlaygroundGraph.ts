import { computed, ref } from 'vue';
import {
  getGraphSnapshot,
  inspectGraphNode,
  type GraphEdgeDto,
  type GraphNodeDto,
  type GraphSnapshotDto,
} from '@/lib/playground';

const DEFAULT_NODE_LIMIT = 200;
const DEFAULT_EDGE_LIMIT = 400;

function normalizeLimit(value: string, fallback: number): number {
  const parsed = Number(value.trim());
  if (!Number.isFinite(parsed) || parsed <= 0) {
    return fallback;
  }
  return Math.max(1, Math.floor(parsed));
}

export function usePlaygroundGraph() {
  const snapshot = ref<GraphSnapshotDto | null>(null);
  const selectedNode = ref<GraphNodeDto | null>(null);
  const selectedEdge = ref<GraphEdgeDto | null>(null);
  const error = ref<string | null>(null);
  const isLoading = ref(true);
  const isRefreshing = ref(false);
  const lastLoadedAt = ref<Date | null>(null);
  const nodeLimitInput = ref(String(DEFAULT_NODE_LIMIT));
  const edgeLimitInput = ref(String(DEFAULT_EDGE_LIMIT));
  const activeNodeLimit = ref(DEFAULT_NODE_LIMIT);
  const activeEdgeLimit = ref(DEFAULT_EDGE_LIMIT);

  const graphTitle = computed(() => {
    if (!snapshot.value) {
      return 'GraphClaw graph snapshot';
    }
    return snapshot.value.meta.truncated
      ? 'GraphClaw graph snapshot (bounded)'
      : 'GraphClaw graph snapshot';
  });

  async function loadSnapshot(options?: { nodeLimit?: number; edgeLimit?: number }) {
    const nodeLimit = options?.nodeLimit ?? activeNodeLimit.value;
    const edgeLimit = options?.edgeLimit ?? activeEdgeLimit.value;

    error.value = null;
    isRefreshing.value = true;

    try {
      snapshot.value = await getGraphSnapshot({ nodeLimit, edgeLimit });
      activeNodeLimit.value = nodeLimit;
      activeEdgeLimit.value = edgeLimit;
      selectedNode.value = null;
      selectedEdge.value = null;
      lastLoadedAt.value = new Date();
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to load graph snapshot';
    } finally {
      isLoading.value = false;
      isRefreshing.value = false;
    }
  }

  async function initialize() {
    await loadSnapshot({
      nodeLimit: DEFAULT_NODE_LIMIT,
      edgeLimit: DEFAULT_EDGE_LIMIT,
    });
  }

  async function applyLimits() {
    const nodeLimit = normalizeLimit(nodeLimitInput.value, DEFAULT_NODE_LIMIT);
    const edgeLimit = normalizeLimit(edgeLimitInput.value, DEFAULT_EDGE_LIMIT);
    nodeLimitInput.value = String(nodeLimit);
    edgeLimitInput.value = String(edgeLimit);
    await loadSnapshot({ nodeLimit, edgeLimit });
  }

  async function refresh() {
    await loadSnapshot();
  }

  async function selectNode(nodeId: number) {
    selectedEdge.value = null;
    const fallbackNode = snapshot.value?.nodes.find((node) => node.id === nodeId) ?? null;
    try {
      selectedNode.value = (await inspectGraphNode(nodeId)) ?? fallbackNode;
    } catch {
      selectedNode.value = fallbackNode;
    }
  }

  function selectEdge(edgeId: number) {
    selectedNode.value = null;
    selectedEdge.value = snapshot.value?.edges.find((edge) => edge.id === edgeId) ?? null;
  }

  function clearSelection() {
    selectedNode.value = null;
    selectedEdge.value = null;
  }

  return {
    snapshot,
    selectedNode,
    selectedEdge,
    error,
    isLoading,
    isRefreshing,
    lastLoadedAt,
    nodeLimitInput,
    edgeLimitInput,
    activeNodeLimit,
    activeEdgeLimit,
    graphTitle,
    initialize,
    applyLimits,
    refresh,
    selectNode,
    selectEdge,
    clearSelection,
  };
}
