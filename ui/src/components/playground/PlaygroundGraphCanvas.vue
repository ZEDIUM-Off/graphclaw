<template>
  <Card class="overflow-hidden border-border/70 bg-card/90">
    <CardHeader class="flex-row items-start justify-between gap-4 space-y-0 border-b border-border/70">
      <div class="space-y-1">
        <CardTitle class="text-xl">{{ title }}</CardTitle>
        <CardDescription>
          {{ nodes.length }} nodes, {{ edges.length }} edges, layout {{ layoutMode }}
        </CardDescription>
      </div>

      <div class="grid gap-1 text-right text-xs text-muted-foreground">
        <span>{{ selectedNodeIds.length }} selected nodes</span>
        <span>{{ selectedEdgeId ? `edge #${selectedEdgeId}` : 'no edge selected' }}</span>
      </div>
    </CardHeader>

    <CardContent class="p-0">
      <div
        v-if="isLoading"
        class="grid h-[min(72vh,54rem)] place-items-center text-sm text-muted-foreground"
      >
        Resolving active set...
      </div>
      <div
        v-else-if="nodes.length === 0"
        class="grid h-[min(72vh,54rem)] place-items-center text-sm text-muted-foreground"
      >
        This set resolved to an empty subgraph.
      </div>
      <div
        v-else
        ref="containerRef"
        class="h-[min(72vh,54rem)] w-full bg-[radial-gradient(circle_at_18%_18%,rgba(225,173,1,0.12),transparent_30%),radial-gradient(circle_at_82%_16%,rgba(77,187,161,0.14),transparent_28%),radial-gradient(circle_at_50%_75%,rgba(94,155,255,0.1),transparent_36%),#05101d]"
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
    selectedNodeIds?: number[];
    selectedEdgeId?: number | null;
    layoutMode?: 'grid' | 'circlepack' | 'radial';
    appearanceMode?: 'labels' | 'sets' | 'density';
  }>(),
  {
    isLoading: false,
    selectedNodeIds: () => [],
    selectedEdgeId: null,
    layoutMode: 'circlepack',
    appearanceMode: 'labels',
  },
);

const emit = defineEmits<{
  nodeToggle: [nodeId: number];
  edgeClick: [edgeId: number];
  stageClick: [];
}>();

const containerRef = ref<HTMLDivElement | null>(null);
let renderer: Sigma | null = null;

function nodeLabel(node: GraphNodeDto): string {
  const name = node.properties.name;
  if (typeof name === 'string' && name.trim().length > 0) {
    return name;
  }
  return node.labels[0] ?? String(node.id);
}

function nodeDegree(nodeId: number): number {
  return props.edges.filter((edge) => edge.from_id === nodeId || edge.to_id === nodeId).length;
}

function nodeColor(node: GraphNodeDto): string {
  const isSelected = props.selectedNodeIds.includes(node.id);
  const isSetNode = node.labels.includes('Set');

  if (isSelected) {
    return '#ffd166';
  }

  if (props.appearanceMode === 'sets') {
    return isSetNode ? '#f97316' : '#3b82f6';
  }

  if (props.appearanceMode === 'density') {
    const degree = nodeDegree(node.id);
    if (degree >= 4) return '#ef4444';
    if (degree >= 2) return '#f59e0b';
    return '#38bdf8';
  }

  if (isSetNode) return '#f97316';
  if (node.labels.includes('Business')) return '#22c55e';
  if (node.labels.includes('Brand')) return '#ec4899';
  if (node.labels.includes('Concept')) return '#60a5fa';
  return '#94a3b8';
}

function nodeSize(node: GraphNodeDto): number {
  const base = props.selectedNodeIds.includes(node.id) ? 18 : 10;
  const degreeBonus = Math.min(nodeDegree(node.id), 5);
  return base + degreeBonus;
}

function edgeColor(edge: GraphEdgeDto): string {
  return props.selectedEdgeId === edge.id ? '#ffd166' : '#4b5a74';
}

function assignLayout(graph: Graph) {
  const nodeIds = graph.nodes();
  const total = nodeIds.length;
  const columnCount = Math.max(1, Math.ceil(Math.sqrt(total)));

  nodeIds.forEach((nodeId, index) => {
    if (props.layoutMode === 'grid') {
      graph.mergeNodeAttributes(nodeId, {
        x: (index % columnCount) * 16,
        y: Math.floor(index / columnCount) * 16,
      });
      return;
    }

    if (props.layoutMode === 'radial') {
      const angle = (index / Math.max(total, 1)) * Math.PI * 2;
      const radius = 16 + (index % 5) * 10;
      graph.mergeNodeAttributes(nodeId, {
        x: Math.cos(angle) * radius,
        y: Math.sin(angle) * radius,
      });
      return;
    }
  });

  if (props.layoutMode === 'circlepack') {
    try {
      circlepack.assign(graph, {
        hierarchyAttributes: ['primaryLabel'],
        scale: Math.max(80, total * 8),
      });
    } catch {
      // Keep deterministic coordinates from the fallback assignment.
    }
  }
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

  props.nodes.forEach((node) => {
    graph.addNode(String(node.id), {
      x: 0,
      y: 0,
      primaryLabel: node.labels[0] ?? 'Node',
      size: nodeSize(node),
      label: nodeLabel(node),
      color: nodeColor(node),
    });
  });

  assignLayout(graph);

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
    labelDensity: 0.09,
    labelGridCellSize: 80,
    defaultEdgeType: 'arrow',
    zIndex: true,
  });

  renderer.on('clickNode', ({ node }: { node: string }) => {
    emit('nodeToggle', Number(node));
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
  () => [
    props.nodes,
    props.edges,
    props.isLoading,
    props.selectedNodeIds,
    props.selectedEdgeId,
    props.layoutMode,
    props.appearanceMode,
  ],
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
