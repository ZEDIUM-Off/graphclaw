import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { ViewTemplateDto } from '@/lib/api';

const {
  playgroundGetGraph,
  playgroundListViews,
  playgroundGetView,
  playgroundCreateNode,
  playgroundCreateEdge,
  playgroundGetSubgraph,
  playgroundInspectNode,
  playgroundCreateView,
  playgroundUpdateView,
  playgroundResolve,
  playgroundExport,
} = vi.hoisted(() => ({
  playgroundGetGraph: vi.fn(),
  playgroundListViews: vi.fn<() => Promise<string[]>>(),
  playgroundGetView: vi.fn<(id: string) => Promise<ViewTemplateDto>>(),
  playgroundCreateNode: vi.fn(),
  playgroundCreateEdge: vi.fn(),
  playgroundGetSubgraph: vi.fn(),
  playgroundInspectNode: vi.fn(),
  playgroundCreateView: vi.fn(),
  playgroundUpdateView: vi.fn(),
  playgroundResolve: vi.fn(),
  playgroundExport: vi.fn(),
}));

vi.mock('@/lib/api', () => ({
  playgroundGetGraph,
  playgroundCreateNode,
  playgroundCreateEdge,
  playgroundGetView,
  playgroundGetSubgraph,
  playgroundInspectNode,
  playgroundListViews,
  playgroundCreateView,
  playgroundUpdateView,
  playgroundResolve,
  playgroundExport,
}));

vi.mock('@/components/playground/PlaygroundCanvas', () => ({
  default: ({
    title,
    nodes,
    edges,
    isLoading,
    onNodeClick,
  }: {
    title: string;
    nodes: { id: number }[];
    edges: { id: number }[];
    isLoading?: boolean;
    onNodeClick?: (nodeId: number) => void;
  }) => (
    <div data-testid="mock-canvas">
      {isLoading ? 'loading' : `${title} :: ${nodes.length} nodes / ${edges.length} edges`}
      <button type="button" onClick={() => onNodeClick?.(1)}>
        Select node 1
      </button>
    </div>
  ),
}));

vi.mock('@/components/playground/PlaygroundInspector', () => ({
  default: ({
    exportText,
    onUseSelectionInViewEditor,
    onUseSelectionAsResolveAnchors,
    onMergeSelectionIntoLoadedView,
  }: {
    exportText?: string | null;
    onUseSelectionInViewEditor?: () => void;
    onUseSelectionAsResolveAnchors?: () => void;
    onMergeSelectionIntoLoadedView?: () => void;
  }) => (
    <div data-testid="mock-inspector">
      <div>{exportText ?? 'inspector-empty'}</div>
      <button type="button" onClick={onUseSelectionInViewEditor}>
        Use selection in view editor
      </button>
      <button type="button" onClick={onUseSelectionAsResolveAnchors}>
        Use selection as resolve anchors
      </button>
      <button type="button" onClick={onMergeSelectionIntoLoadedView}>
        Merge selection into loaded view
      </button>
    </div>
  ),
}));

import Playground, {
  formatTemplate,
  formatSelectionAnchors,
  mergeNodeIds,
  parseBoundAnchors,
  parseCsvIds,
  parseOperations,
  parseViewOperation,
  splitList,
  upsertEdge,
  upsertNode,
} from './Playground';

describe('Playground page', () => {
  afterEach(() => {
    cleanup();
  });

  beforeEach(() => {
    playgroundGetGraph.mockReset();
    playgroundListViews.mockReset();
    playgroundGetView.mockReset();
    playgroundCreateNode.mockReset();
    playgroundCreateEdge.mockReset();
    playgroundGetSubgraph.mockReset();
    playgroundInspectNode.mockReset();
    playgroundCreateView.mockReset();
    playgroundUpdateView.mockReset();
    playgroundResolve.mockReset();
    playgroundExport.mockReset();

    playgroundGetGraph.mockResolvedValue({
      nodes: [{ id: 1, labels: ['Business'], properties: { name: 'Pricing' } }],
      edges: [],
      meta: { truncated: false, node_limit: 200, edge_limit: 400 },
    });
    playgroundListViews.mockResolvedValue(['view_core', 'view_business']);
    playgroundGetView.mockResolvedValue({
      id: 'view_core',
      name: 'Core view',
      kind: 'semantic',
      description: 'Core concepts',
      extends: ['view_base'],
      selectors: { node_ids: [1, 2], label: 'Business', props: { workspace: 'HIGHFINITY' } },
      filters: ['label:Business'],
      operations: [{ op: 'expand', relation_type: 'RELATES_TO', depth: 1 }],
      cost_limit: 12,
    });
    playgroundInspectNode.mockResolvedValue({
      id: 1,
      labels: ['Business'],
      properties: { name: 'Pricing' },
    });
  });

  it('parses and formats helper values deterministically', () => {
    expect(parseCsvIds('1, 2, nope, 3')).toEqual([1, 2, 3]);
    expect(splitList('a,\nb, c')).toEqual(['a', 'b', 'c']);
    expect(parseBoundAnchors('1, 2')).toEqual({ node_ids: [1, 2] });
    expect(
      parseViewOperation({ op: 'slice', limit: 4, order: 'name_asc' }),
    ).toEqual({ op: 'slice', limit: 4, order: 'name_asc' });
    expect(parseOperations('[{"op":"filter_nodes","predicate":"label:Business"}]')).toEqual([
      { op: 'filter_nodes', predicate: 'label:Business' },
    ]);
    expect(
      upsertNode(
        [{ id: 2, labels: [], properties: {} }],
        { id: 1, labels: [], properties: {} },
      ).map((node) => node.id),
    ).toEqual([1, 2]);
    expect(
      upsertEdge(
        [{ id: 4, from_id: 1, to_id: 2, rel_type: 'REL', properties: {} }],
        { id: 3, from_id: 1, to_id: 2, rel_type: 'REL', properties: {} },
      ).map((edge) => edge.id),
    ).toEqual([3, 4]);
    expect(mergeNodeIds([3, 1], [2, 3])).toEqual([1, 2, 3]);
    expect(formatSelectionAnchors([1, 2])).toBe('{\n  "node_ids": [\n    1,\n    2\n  ]\n}');

    expect(
      formatTemplate({
        id: 'view_business',
        name: 'Business',
        kind: 'semantic',
        description: 'Business view',
        extends: ['view_core'],
        selectors: { node_ids: [1], label: 'Business', props: { workspace: 'HIGHFINITY' } },
        filters: ['label:Business'],
        operations: [{ op: 'expand', relation_type: 'RELATES_TO', depth: 2 }],
        cost_limit: 6,
      }),
    ).toEqual({
      extendsText: 'view_core',
      nodeIdsText: '1',
      selectorLabel: 'Business',
      selectorPropsText: '{\n  "workspace": "HIGHFINITY"\n}',
      filtersText: 'label:Business',
      operationsText: '[\n  {\n    "op": "expand",\n    "relation_type": "RELATES_TO",\n    "depth": 2\n  }\n]',
      costLimitText: '6',
    });
  });

  it('loads the template list on mount and lets the user reload a template', async () => {
    render(<Playground />);

    expect(await screen.findByText('view_core')).toBeInTheDocument();
    expect(screen.getByText('view_business')).toBeInTheDocument();
    expect(screen.getByTestId('mock-canvas')).toHaveTextContent('Playground graph :: 1 nodes / 0 edges');

    fireEvent.click(screen.getByText('view_core'));

    await waitFor(() => {
      expect(playgroundGetView).toHaveBeenCalledWith('view_core');
    });
    expect(screen.getByText('Loaded template view_core into the editor.')).toBeInTheDocument();
  });

  it('shows a loading state before the initial graph snapshot resolves', async () => {
    let resolveGraph: ((value: { nodes: []; edges: []; meta: { truncated: false; node_limit: 200; edge_limit: 400 } }) => void) | undefined;
    playgroundGetGraph.mockReturnValue(
      new Promise((resolve) => {
        resolveGraph = resolve;
      }),
    );

    render(<Playground />);

    expect(screen.getByTestId('mock-canvas')).toHaveTextContent('loading');

    resolveGraph?.({
      nodes: [],
      edges: [],
      meta: { truncated: false, node_limit: 200, edge_limit: 400 },
    });

    await waitFor(() => {
      expect(screen.getByTestId('mock-canvas')).toHaveTextContent('Playground graph :: 0 nodes / 0 edges');
    });
  });

  it('injects the current selection into the view editor and resolve anchors', async () => {
    render(<Playground />);
    await screen.findByText('view_core');

    fireEvent.click(screen.getByText('Select node 1'));

    await waitFor(() => {
      expect(playgroundInspectNode).toHaveBeenCalledWith(1);
    });

    fireEvent.click(screen.getByText('Use selection in view editor'));
    expect(screen.getByDisplayValue('1')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Use selection as resolve anchors'));
    expect(
      screen.getByPlaceholderText('{"node_ids":[1,2],"label":"Business"}'),
    ).toHaveValue('{\n  "node_ids": [\n    1\n  ]\n}');
  });
});
