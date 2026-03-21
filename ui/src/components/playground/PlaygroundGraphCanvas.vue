<template>
  <Card class="overflow-hidden">
    <CardHeader class="flex-row items-start justify-between gap-4 space-y-0 border-b border-border/70">
      <div>
        <CardTitle class="text-xl">{{ title }}</CardTitle>
        <CardDescription>{{ nodes.length }} nodes, {{ edges.length }} edges</CardDescription>
      </div>

      <div class="grid gap-1 text-right text-xs text-muted-foreground">
        <span>{{ selectedNodeId ? `selected node #${selectedNodeId}` : 'no node selected' }}</span>
        <span>Click the background to clear selection</span>
      </div>
    </CardHeader>

    <CardContent class="p-0">
      <div
        v-if="isLoading"
        class="grid h-[min(72vh,52rem)] place-items-center text-sm text-muted-foreground"
      >
        Loading graph snapshot...
      </div>
      <div
        v-else-if="nodes.length === 0"
        class="grid h-[min(72vh,52rem)] place-items-center text-sm text-muted-foreground"
      >
        No nodes loaded yet. Refresh the gateway snapshot or increase the graph limits.
      </div>
      <div
        v-else
        ref="containerRef"
        class="h-[min(72vh,52rem)] w-full bg-[radial-gradient(circle_at_20%_20%,rgba(101,161,255,0.08),transparent_30%),radial-gradient(circle_at_80%_20%,rgba(124,224,195,0.08),transparent_28%),#020814]"
      />
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue';
import Graph from 'graphology';
import circlepack from 'graphology-layout/circlepack';
import Sigma from 'sigma';
import type { GraphEdgeDto, GraphNodeDto } from '@/lib/playground';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

const props = withDefaults(
  defineProps<{
    title: string;
    nodes: GraphNodeDto[];
    edges: GraphEdgeDto[];
    isLoading?: boolean;
    selectedNodeId?: number | null;
    selectedEdgeId?: number | null;
  }>(),
  {
    isLoading: false,
    selectedNodeId: null,
    selectedEdgeId: null,
  },
);

const emit = defineEmits<{
  nodeClick: [nodeId: number];
  edgeClick: [edgeId: number];
  stageClick: [];
}>();

const containerRef = ref<HTMLDivElement | null>(null);
let renderer: Sigma | null = null;

function nodeLabel(node: GraphNodeDto): string {
  const name = node.properties.name;
  return typeof name === 'string' && name.trim().length > 0
    ? name
    : node.labels[0] ?? String(node.id);
}

function nodeColor(node: GraphNodeDto): string {
  if (props.selectedNodeId === node.id) {
    return '#f4d35e';
  }
  if (node.labels.includes('Business')) {
    return '#4ade80';
  }
  if (node.labels.includes('Brand')) {
    return '#f97393';
  }
  if (node.labels.includes('Concept')) {
    return '#60a5fa';
  }
  return '#94a3b8';
}

function edgeColor(edge: GraphEdgeDto): string {
  return props.selectedEdgeId === edge.id ? '#f4d35e' : '#516078';
}

function renderGraph() {
  if (renderer) {
    renderer.kill();
    renderer = null;
  }

  const container = containerRef.value;
  if (!container || props.isLoading || props.nodes.length === 0) {
    return;
  }

  const graph = new Graph({ multi: true, type: 'directed' });

  props.nodes.forEach((node, index) => {
    const columnCount = Math.max(1, Math.ceil(Math.sqrt(props.nodes.length)));
    const x = (index % columnCount) * 16;
    const y = Math.floor(index / columnCount) * 16;
    graph.addNode(String(node.id), {
      x,
      y,
      primaryLabel: node.labels[0] ?? 'Node',
      size: props.selectedNodeId === node.id ? 18 : 11,
      label: nodeLabel(node),
      color: nodeColor(node),
    });
  });

  try {
    circlepack.assign(graph, {
      hierarchyAttributes: ['primaryLabel'],
      scale: Math.max(80, props.nodes.length * 8),
    });
  } catch {
    // Keep deterministic fallback coordinates.
  }

  props.edges.forEach((edge) => {
    const source = String(edge.from_id);
    const target = String(edge.to_id);
    if (!graph.hasNode(source) || !graph.hasNode(target)) {
      return;
    }
    graph.addEdgeWithKey(String(edge.id), source, target, {
      label: edge.rel_type,
      size: props.selectedEdgeId === edge.id ? 4 : 2,
      color: edgeColor(edge),
    });
  });

  renderer = new Sigma(graph, container, {
    renderEdgeLabels: true,
    enableEdgeClickEvents: true,
    labelDensity: 0.08,
    labelGridCellSize: 80,
    defaultEdgeType: 'arrow',
    zIndex: true,
  });

  renderer.on('clickNode', ({ node }: { node: string }) => {
    emit('nodeClick', Number(node));
  });
  renderer.on('clickEdge', ({ edge }: { edge: string }) => {
    emit('edgeClick', Number(edge));
  });
  renderer.on('clickStage', () => {
    emit('stageClick');
  });
  renderer.refresh();
}

watch(
  () => [props.nodes, props.edges, props.isLoading, props.selectedNodeId, props.selectedEdgeId],
  () => {
    renderGraph();
  },
  { deep: true, immediate: true },
);

onMounted(() => {
  renderGraph();
});

onBeforeUnmount(() => {
  if (renderer) {
    renderer.kill();
    renderer = null;
  }
});
</script>
