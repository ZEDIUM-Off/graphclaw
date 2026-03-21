export interface GraphNodeDto {
  id: number;
  labels: string[];
  properties: Record<string, unknown>;
}

export interface GraphEdgeDto {
  id: number;
  from_id: number;
  to_id: number;
  rel_type: string;
  properties: Record<string, unknown>;
}

export interface GraphSnapshotMetaDto {
  truncated: boolean;
  node_limit: number;
  edge_limit: number;
}

export interface GraphSnapshotDto {
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  meta: GraphSnapshotMetaDto;
}

async function apiFetch<T>(path: string): Promise<T> {
  const response = await fetch(path);
  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(`API ${response.status}: ${text || response.statusText}`);
  }
  return response.json() as Promise<T>;
}

export function getGraphSnapshot(options: {
  nodeLimit?: number;
  edgeLimit?: number;
} = {}): Promise<GraphSnapshotDto> {
  const params = new URLSearchParams();
  if (typeof options.nodeLimit === 'number') {
    params.set('node_limit', String(options.nodeLimit));
  }
  if (typeof options.edgeLimit === 'number') {
    params.set('edge_limit', String(options.edgeLimit));
  }
  const suffix = params.toString();
  return apiFetch<GraphSnapshotDto>(`/api/playground/graph${suffix ? `?${suffix}` : ''}`);
}

export function inspectGraphNode(id: number): Promise<GraphNodeDto | null> {
  return apiFetch<GraphNodeDto | { error: string }>(`/api/playground/graph/nodes/${id}`).then(
    (data) => {
      if (data && typeof data === 'object' && 'error' in data) {
        return null;
      }
      return data as GraphNodeDto;
    },
  );
}
