import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import PlaygroundInspector, { renderJson } from './PlaygroundInspector';
import type { GraphEdgeDto, GraphNodeDto, ResolvedViewDto } from '@/lib/api';

describe('PlaygroundInspector', () => {
  afterEach(() => {
    cleanup();
  });

  it('renders placeholder states when nothing is selected yet', () => {
    render(<PlaygroundInspector />);

    expect(screen.getByText('No node selection yet.')).toBeInTheDocument();
    expect(screen.getByText('No node selected.')).toBeInTheDocument();
    expect(screen.getByText('No edge selected.')).toBeInTheDocument();
    expect(screen.getByText('No resolved view yet.')).toBeInTheDocument();
    expect(screen.getByText('No export generated yet.')).toBeInTheDocument();
  });

  it('renders selected graph elements, resolution trace, and export text', () => {
    const selectedNode: GraphNodeDto = {
      id: 1,
      labels: ['Business'],
      properties: { name: 'Pricing' },
    };
    const selectedEdge: GraphEdgeDto = {
      id: 2,
      from_id: 1,
      to_id: 3,
      rel_type: 'RELATES_TO',
      properties: { weight: 1 },
    };
    const resolvedView: ResolvedViewDto = {
      view_id: 'view_business',
      view_kind: 'semantic',
      nodes: [selectedNode],
      edges: [selectedEdge],
      composition_trace: ['expand RELATES_TO depth=1'],
      completeness: 'full',
      degradations: ['cost_limit applied'],
      cost_estimate: 4,
    };

    render(
      <PlaygroundInspector
        selectedNode={selectedNode}
        selectedNodeIds={[1, 3]}
        selectedEdge={selectedEdge}
        resolvedView={resolvedView}
        exportText="Structured export"
        canMergeSelectionIntoView
      />,
    );

    expect(screen.getByText('2 nodes selected')).toBeInTheDocument();
    expect(screen.getByText(/id=1 labels=Business/)).toBeInTheDocument();
    expect(screen.getByText(/1 -\[RELATES_TO\]-> 3/)).toBeInTheDocument();
    expect(screen.getByText(/expand RELATES_TO depth=1/)).toBeInTheDocument();
    expect(screen.getByText(/cost_limit applied/)).toBeInTheDocument();
    expect(screen.getByText('Structured export')).toBeInTheDocument();
  });

  it('fires selection action callbacks', () => {
    const useSelectionInViewEditor = vi.fn();
    const useSelectionAsResolveAnchors = vi.fn();
    const mergeSelectionIntoLoadedView = vi.fn();

    render(
      <PlaygroundInspector
        selectedNodeIds={[1]}
        canMergeSelectionIntoView
        onUseSelectionInViewEditor={useSelectionInViewEditor}
        onUseSelectionAsResolveAnchors={useSelectionAsResolveAnchors}
        onMergeSelectionIntoLoadedView={mergeSelectionIntoLoadedView}
      />,
    );

    fireEvent.click(screen.getByText('Use selection in view editor'));
    fireEvent.click(screen.getByText('Use selection as resolve anchors'));
    fireEvent.click(screen.getByText('Merge selection into loaded view'));

    expect(useSelectionInViewEditor).toHaveBeenCalledTimes(1);
    expect(useSelectionAsResolveAnchors).toHaveBeenCalledTimes(1);
    expect(mergeSelectionIntoLoadedView).toHaveBeenCalledTimes(1);
  });

  it('formats json values with stable indentation', () => {
    expect(renderJson({ name: 'Pricing', rank: 1 })).toBe(
      '{\n  "name": "Pricing",\n  "rank": 1\n}',
    );
  });
});
