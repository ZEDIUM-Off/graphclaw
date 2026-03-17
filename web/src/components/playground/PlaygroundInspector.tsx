import type { GraphEdgeDto, GraphNodeDto, ResolvedViewDto } from '@/lib/api';

interface PlaygroundInspectorProps {
  selectedNode?: GraphNodeDto | null;
  selectedNodeIds?: number[];
  selectedEdge?: GraphEdgeDto | null;
  resolvedView?: ResolvedViewDto | null;
  exportText?: string | null;
  canMergeSelectionIntoView?: boolean;
  onUseSelectionInViewEditor?: () => void;
  onUseSelectionAsResolveAnchors?: () => void;
  onMergeSelectionIntoLoadedView?: () => void;
}

export function renderJson(value: unknown): string {
  return JSON.stringify(value, null, 2);
}

export default function PlaygroundInspector({
  selectedNode,
  selectedNodeIds = [],
  selectedEdge,
  resolvedView,
  exportText,
  canMergeSelectionIntoView = false,
  onUseSelectionInViewEditor,
  onUseSelectionAsResolveAnchors,
  onMergeSelectionIntoLoadedView,
}: PlaygroundInspectorProps) {
  return (
    <section className="bg-gray-900 border border-gray-800 rounded-2xl p-4 space-y-4">
      <div>
        <h2 className="text-lg font-semibold text-white">Inspector</h2>
        <p className="text-sm text-gray-400">
          Inspect selected graph elements, the resolved view trace, and the latest export.
        </p>
      </div>

      <div className="rounded-xl border border-gray-800 bg-gray-950/60 p-3 space-y-3">
        <div>
          <h3 className="text-sm font-medium text-amber-300 mb-1">Selection</h3>
          <p className="text-sm text-gray-300">
            {selectedNodeIds.length > 0
              ? `${selectedNodeIds.length} node${selectedNodeIds.length > 1 ? 's' : ''} selected`
              : 'No node selection yet.'}
          </p>
          {selectedNodeIds.length > 0 && (
            <p className="text-xs text-gray-500 mt-1">node_ids=[{selectedNodeIds.join(', ')}]</p>
          )}
        </div>
        <div className="flex flex-wrap gap-2">
          <button
            type="button"
            onClick={onUseSelectionInViewEditor}
            disabled={selectedNodeIds.length === 0}
            className="px-3 py-1.5 rounded bg-amber-600 text-white text-xs font-medium disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Use selection in view editor
          </button>
          <button
            type="button"
            onClick={onUseSelectionAsResolveAnchors}
            disabled={selectedNodeIds.length === 0}
            className="px-3 py-1.5 rounded bg-violet-600 text-white text-xs font-medium disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Use selection as resolve anchors
          </button>
          <button
            type="button"
            onClick={onMergeSelectionIntoLoadedView}
            disabled={selectedNodeIds.length === 0 || !canMergeSelectionIntoView}
            className="px-3 py-1.5 rounded bg-blue-600 text-white text-xs font-medium disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Merge selection into loaded view
          </button>
        </div>
      </div>

      {selectedNode ? (
        <div>
          <h3 className="text-sm font-medium text-blue-300 mb-2">Selected node</h3>
          <div className="text-sm text-gray-300 mb-2">
            id={selectedNode.id} labels={selectedNode.labels.join(', ') || 'none'}
          </div>
          <pre className="bg-gray-950 border border-gray-800 rounded-lg p-3 text-xs text-gray-300 overflow-auto max-h-48">
            {renderJson(selectedNode.properties)}
          </pre>
        </div>
      ) : (
        <div className="text-sm text-gray-500">No node selected.</div>
      )}

      {selectedEdge ? (
        <div>
          <h3 className="text-sm font-medium text-emerald-300 mb-2">Selected edge</h3>
          <div className="text-sm text-gray-300 mb-2">
            id={selectedEdge.id} {selectedEdge.from_id} -[{selectedEdge.rel_type}]-&gt; {selectedEdge.to_id}
          </div>
          <pre className="bg-gray-950 border border-gray-800 rounded-lg p-3 text-xs text-gray-300 overflow-auto max-h-40">
            {renderJson(selectedEdge.properties)}
          </pre>
        </div>
      ) : (
        <div className="text-sm text-gray-500">No edge selected.</div>
      )}

      {resolvedView ? (
        <div>
          <h3 className="text-sm font-medium text-violet-300 mb-2">Resolved view</h3>
          <div className="text-sm text-gray-300 space-y-1 mb-3">
            <div>id={resolvedView.view_id}</div>
            <div>kind={resolvedView.view_kind ?? 'n/a'}</div>
            <div>
              nodes={resolvedView.nodes.length}, edges={resolvedView.edges.length}, cost=
              {resolvedView.cost_estimate ?? 'n/a'}
            </div>
            <div>completeness={resolvedView.completeness ?? 'n/a'}</div>
          </div>
          <div className="grid gap-3 md:grid-cols-2">
            <div>
              <h4 className="text-xs uppercase tracking-wide text-gray-400 mb-2">Trace</h4>
              <pre className="bg-gray-950 border border-gray-800 rounded-lg p-3 text-xs text-gray-300 overflow-auto max-h-40 whitespace-pre-wrap">
                {resolvedView.composition_trace?.length
                  ? resolvedView.composition_trace.join('\n')
                  : 'No trace.'}
              </pre>
            </div>
            <div>
              <h4 className="text-xs uppercase tracking-wide text-gray-400 mb-2">Degradations</h4>
              <pre className="bg-gray-950 border border-gray-800 rounded-lg p-3 text-xs text-gray-300 overflow-auto max-h-40 whitespace-pre-wrap">
                {resolvedView.degradations?.length
                  ? resolvedView.degradations.join('\n')
                  : 'No degradations.'}
              </pre>
            </div>
          </div>
        </div>
      ) : (
        <div className="text-sm text-gray-500">No resolved view yet.</div>
      )}

      <div>
        <h3 className="text-sm font-medium text-amber-300 mb-2">Latest export</h3>
        <pre className="bg-gray-950 border border-gray-800 rounded-lg p-3 text-xs text-gray-300 overflow-auto max-h-64 whitespace-pre-wrap">
          {exportText ?? 'No export generated yet.'}
        </pre>
      </div>
    </section>
  );
}
