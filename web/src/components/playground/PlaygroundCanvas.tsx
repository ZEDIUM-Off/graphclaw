import { useEffect, useRef } from 'react';
import Graph from 'graphology';
import circlepack from 'graphology-layout/circlepack';
import Sigma from 'sigma';
import type { GraphEdgeDto, GraphNodeDto } from '@/lib/api';

interface PlaygroundCanvasProps {
  title: string;
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  isLoading?: boolean;
  emphasizedNodeIds?: number[];
  emphasizedEdgeIds?: number[];
  selectedNodeIds?: number[];
  primarySelectedNodeId?: number | null;
  selectedEdgeId?: number | null;
  onNodeClick?: (nodeId: number) => void;
  onEdgeClick?: (edgeId: number) => void;
}

export function nodeLabel(node: GraphNodeDto): string {
  const name = node.properties.name;
  return typeof name === 'string' && name.trim().length > 0
    ? name
    : node.labels[0] ?? String(node.id);
}

export function nodeColor(
  node: GraphNodeDto,
  selectedNodeIds: Set<number>,
  emphasizedNodeIds: Set<number>,
  primarySelectedNodeId: number | null | undefined,
): string {
  if (primarySelectedNodeId === node.id) {
    return '#facc15';
  }
  if (selectedNodeIds.has(node.id)) {
    return '#fb923c';
  }
  if (emphasizedNodeIds.has(node.id)) {
    return '#60a5fa';
  }
  if (node.labels.includes('Business')) {
    return '#34d399';
  }
  if (node.labels.includes('Brand')) {
    return '#fb7185';
  }
  return '#94a3b8';
}

export function edgeColor(
  edge: GraphEdgeDto,
  emphasizedEdgeIds: Set<number>,
  selectedEdgeId: number | null | undefined,
): string {
  if (selectedEdgeId === edge.id) {
    return '#facc15';
  }
  if (emphasizedEdgeIds.has(edge.id)) {
    return '#60a5fa';
  }
  return '#475569';
}

export default function PlaygroundCanvas({
  title,
  nodes,
  edges,
  isLoading = false,
  emphasizedNodeIds = [],
  emphasizedEdgeIds = [],
  selectedNodeIds = [],
  primarySelectedNodeId,
  selectedEdgeId,
  onNodeClick,
  onEdgeClick,
}: PlaygroundCanvasProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || nodes.length === 0) {
      return;
    }

    const graph = new Graph({ multi: true, type: 'directed' });
    const selectedNodes = new Set(selectedNodeIds);
    const highlightedNodes = new Set(emphasizedNodeIds);
    const highlightedEdges = new Set(emphasizedEdgeIds);

    nodes.forEach((node, index) => {
      const columnCount = Math.max(1, Math.ceil(Math.sqrt(nodes.length)));
      const x = (index % columnCount) * 16;
      const y = Math.floor(index / columnCount) * 16;
      graph.addNode(String(node.id), {
        x,
        y,
        primaryLabel: node.labels[0] ?? 'Node',
        size:
          primarySelectedNodeId === node.id
            ? 18
            : selectedNodes.has(node.id)
              ? 15
              : highlightedNodes.has(node.id)
                ? 14
                : 11,
        label: nodeLabel(node),
        color: nodeColor(node, selectedNodes, highlightedNodes, primarySelectedNodeId),
      });
    });

    try {
      circlepack.assign(graph, {
        hierarchyAttributes: ['primaryLabel'],
        scale: Math.max(80, nodes.length * 8),
      });
    } catch {
      // Keep the deterministic fallback grid when the layout cannot run.
    }

    edges.forEach((edge) => {
      const source = String(edge.from_id);
      const target = String(edge.to_id);
      if (!graph.hasNode(source) || !graph.hasNode(target)) {
        return;
      }
      graph.addEdgeWithKey(String(edge.id), source, target, {
        label: edge.rel_type,
        size: selectedEdgeId === edge.id ? 4 : highlightedEdges.has(edge.id) ? 3 : 2,
        color: edgeColor(edge, highlightedEdges, selectedEdgeId),
      });
    });

    const renderer = new Sigma(graph, container, {
      renderEdgeLabels: true,
      enableEdgeEvents: true,
      labelDensity: 0.08,
      labelGridCellSize: 80,
      defaultEdgeType: 'arrow',
      zIndex: true,
    });

    const handleNodeClick = ({ node }: { node: string }) => {
      onNodeClick?.(Number(node));
    };
    const handleEdgeClick = ({ edge }: { edge: string }) => {
      onEdgeClick?.(Number(edge));
    };

    renderer.on('clickNode', handleNodeClick);
    renderer.on('clickEdge', handleEdgeClick);
    renderer.refresh();

    return () => {
      renderer.kill();
    };
  }, [
    edges,
    emphasizedEdgeIds,
    emphasizedNodeIds,
    isLoading,
    nodes,
    onEdgeClick,
    onNodeClick,
    primarySelectedNodeId,
    selectedNodeIds,
    selectedEdgeId,
  ]);

  return (
    <section className="bg-gray-900 border border-gray-800 rounded-2xl overflow-hidden min-h-136">
      <div className="px-4 py-3 border-b border-gray-800 flex items-center justify-between">
        <div>
          <h2 className="text-white text-lg font-semibold">{title}</h2>
          <p className="text-xs text-gray-400">
            {nodes.length} nodes, {edges.length} edges
          </p>
        </div>
        <div className="text-right text-xs text-gray-400">
          <div>{selectedNodeIds.length} selected nodes</div>
          <div>Ctrl/Cmd + click to extend selection</div>
        </div>
      </div>

      {isLoading ? (
        <div className="h-120 flex items-center justify-center text-sm text-gray-500">
          Loading graph snapshot...
        </div>
      ) : nodes.length === 0 ? (
        <div className="h-120 flex items-center justify-center text-sm text-gray-500">
          No nodes loaded yet. Create nodes, fetch a subgraph, or resolve a view to render the
          graph.
        </div>
      ) : (
        <div ref={containerRef} className="h-120 w-full bg-gray-950" />
      )}
    </section>
  );
}
