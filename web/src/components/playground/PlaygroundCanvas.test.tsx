import { act, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import PlaygroundCanvas, { edgeColor, nodeColor, nodeLabel } from './PlaygroundCanvas';
import type { GraphEdgeDto, GraphNodeDto } from '@/lib/api';

type GraphNodeAttributes = {
  x: number;
  y: number;
  size: number;
  label: string;
  color: string;
  primaryLabel?: string;
};

type GraphEdgeAttributes = {
  source: string;
  target: string;
  label: string;
  size: number;
  color: string;
};

type SigmaSettings = {
  renderEdgeLabels: boolean;
  enableEdgeEvents: boolean;
  labelDensity: number;
  labelGridCellSize: number;
  defaultEdgeType: string;
  zIndex: boolean;
};

type SigmaEventPayload = { node?: string; edge?: string };
type SigmaHandler = (payload: SigmaEventPayload) => void;

const { MockGraphClass, MockSigmaClass, sigmaInstances } = vi.hoisted(() => {
  class HoistedMockGraph {
    readonly nodes = new Map<string, GraphNodeAttributes>();
    readonly edges = new Map<string, GraphEdgeAttributes>();

    constructor(_options: { multi: boolean; type: string }) {}

    addNode(id: string, attributes: GraphNodeAttributes): void {
      this.nodes.set(id, attributes);
    }

    hasNode(id: string): boolean {
      return this.nodes.has(id);
    }

    addEdgeWithKey(
      id: string,
      source: string,
      target: string,
      attributes: Omit<GraphEdgeAttributes, 'source' | 'target'>,
    ): void {
      this.edges.set(id, { source, target, ...attributes });
    }
  }

  class HoistedMockSigma {
    readonly graph: HoistedMockGraph;
    readonly container: HTMLElement;
    readonly settings: SigmaSettings;
    readonly handlers = new Map<string, SigmaHandler[]>();
    killed = false;
    refreshed = false;

    constructor(graph: HoistedMockGraph, container: HTMLElement, settings: SigmaSettings) {
      this.graph = graph;
      this.container = container;
      this.settings = settings;
      sigmaInstances.push(this);
    }

    on(eventName: string, handler: SigmaHandler): void {
      const handlers = this.handlers.get(eventName) ?? [];
      handlers.push(handler);
      this.handlers.set(eventName, handlers);
    }

    refresh(): void {
      this.refreshed = true;
    }

    kill(): void {
      this.killed = true;
    }

    trigger(eventName: string, payload: SigmaEventPayload): void {
      for (const handler of this.handlers.get(eventName) ?? []) {
        handler(payload);
      }
    }
  }

  const sigmaInstances: HoistedMockSigma[] = [];

  return {
    MockGraphClass: HoistedMockGraph,
    MockSigmaClass: HoistedMockSigma,
    sigmaInstances,
  };
});

vi.mock('graphology', () => ({ default: MockGraphClass }));
vi.mock('sigma', () => ({ default: MockSigmaClass }));
vi.mock('graphology-layout/circlepack', () => ({
  default: {
    assign: vi.fn(),
  },
}));

describe('PlaygroundCanvas', () => {
  afterEach(() => {
    sigmaInstances.length = 0;
  });

  it('renders an empty state without creating a sigma renderer', () => {
    render(<PlaygroundCanvas title="Empty graph" nodes={[]} edges={[]} />);

    expect(screen.getByText(/No nodes loaded yet/i)).toBeInTheDocument();
    expect(sigmaInstances).toHaveLength(0);
  });

  it('renders a dedicated loading state before graph data is ready', () => {
    render(<PlaygroundCanvas title="Loading graph" nodes={[]} edges={[]} isLoading />);

    expect(screen.getByText(/Loading graph snapshot/i)).toBeInTheDocument();
    expect(sigmaInstances).toHaveLength(0);
  });

  it('creates a graph, skips dangling edges, and wires click events', () => {
    const onNodeClick = vi.fn<(nodeId: number) => void>();
    const onEdgeClick = vi.fn<(edgeId: number) => void>();
    const nodes: GraphNodeDto[] = [
      { id: 1, labels: ['Business'], properties: { name: 'Pricing' } },
      { id: 2, labels: ['Brand'], properties: {} },
    ];
    const edges: GraphEdgeDto[] = [
      { id: 10, from_id: 1, to_id: 2, rel_type: 'RELATES_TO', properties: {} },
      { id: 11, from_id: 1, to_id: 3, rel_type: 'BROKEN', properties: {} },
    ];

    const view = render(
      <PlaygroundCanvas
        title="Playground graph"
        nodes={nodes}
        edges={edges}
        emphasizedNodeIds={[2]}
        emphasizedEdgeIds={[10]}
        selectedNodeIds={[1, 2]}
        primarySelectedNodeId={1}
        selectedEdgeId={10}
        onNodeClick={onNodeClick}
        onEdgeClick={onEdgeClick}
      />,
    );

    const instance = sigmaInstances[0];
    expect(instance).toBeDefined();
    expect(instance?.graph.nodes.size).toBe(2);
    expect(instance?.graph.nodes.get('1')?.label).toBe('Pricing');
    expect(instance?.graph.edges.has('10')).toBe(true);
    expect(instance?.graph.edges.has('11')).toBe(false);
    expect(instance?.settings.enableEdgeEvents).toBe(true);
    expect(instance?.refreshed).toBe(true);

    act(() => {
      instance?.trigger('clickNode', { node: '1' });
      instance?.trigger('clickEdge', { edge: '10' });
    });

    expect(onNodeClick).toHaveBeenCalledWith(1);
    expect(onEdgeClick).toHaveBeenCalledWith(10);

    view.unmount();
    expect(instance?.killed).toBe(true);
  });

  it('exposes deterministic label and color helpers', () => {
    const businessNode: GraphNodeDto = {
      id: 3,
      labels: ['Business'],
      properties: { name: 'Margin' },
    };
    const fallbackNode: GraphNodeDto = {
      id: 4,
      labels: [],
      properties: {},
    };
    const edge: GraphEdgeDto = {
      id: 20,
      from_id: 3,
      to_id: 4,
      rel_type: 'USES',
      properties: {},
    };

    expect(nodeLabel(businessNode)).toBe('Margin');
    expect(nodeLabel(fallbackNode)).toBe('4');
    expect(nodeColor(businessNode, new Set<number>(), new Set<number>(), 3)).toBe('#facc15');
    expect(nodeColor(businessNode, new Set<number>([3]), new Set<number>(), null)).toBe(
      '#fb923c',
    );
    expect(nodeColor(businessNode, new Set<number>(), new Set<number>([3]), null)).toBe(
      '#60a5fa',
    );
    expect(edgeColor(edge, new Set<number>(), 20)).toBe('#facc15');
    expect(edgeColor(edge, new Set<number>([20]), null)).toBe('#60a5fa');
  });
});
