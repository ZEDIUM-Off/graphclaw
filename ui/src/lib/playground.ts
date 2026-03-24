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

export interface SetSelectorsDto {
  node_ids: number[];
  label?: string | null;
  props: Record<string, unknown>;
}

export interface SetOperationDto {
  op: string;
  [key: string]: unknown;
}

export interface SetDefinitionDto {
  id: string;
  name: string;
  kind: string;
  description: string;
  extends: string[];
  selectors: SetSelectorsDto;
  filters: string[];
  operations: SetOperationDto[];
  cost_limit?: number | null;
}

export interface ResolvedSetDto {
  set_id: string;
  set_kind?: string | null;
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  composition_trace: string[];
  completeness?: string | null;
  degradations: string[];
  cost_estimate?: number | null;
}

export interface CreateSetFromSelectionRequest {
  name: string;
  description: string;
  kind?: string;
  node_ids: number[];
  cost_limit?: number | null;
}

export interface CreateNodeRequest {
  labels: string[];
  properties: Record<string, unknown>;
}

export interface CreateEdgeRequest {
  from_id: number;
  to_id: number;
  rel_type: string;
  properties: Record<string, unknown>;
}

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(path, init);
  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(`API ${response.status}: ${text || response.statusText}`);
  }
  return response.json() as Promise<T>;
}

export function getSets(): Promise<SetDefinitionDto[]> {
  return apiFetch<SetDefinitionDto[]>('/api/playground/sets');
}

export function getSet(id: string): Promise<SetDefinitionDto> {
  return apiFetch<SetDefinitionDto>(`/api/playground/sets/${id}`);
}

export function resolveSet(options: {
  setId: string;
  nodeLimit?: number;
  edgeLimit?: number;
}): Promise<ResolvedSetDto> {
  return apiFetch<ResolvedSetDto>('/api/playground/sets/resolve', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      set_id: options.setId,
      anchors: {},
      parameters: {
        ...(typeof options.nodeLimit === 'number' ? { node_limit: options.nodeLimit } : {}),
        ...(typeof options.edgeLimit === 'number' ? { edge_limit: options.edgeLimit } : {}),
      },
      resolution_scope: 'playground',
    }),
  });
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

export function createSetFromSelection(
  payload: CreateSetFromSelectionRequest,
): Promise<SetDefinitionDto> {
  return apiFetch<SetDefinitionDto>('/api/playground/sets/from-selection', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
}

export function createNode(payload: CreateNodeRequest): Promise<GraphNodeDto> {
  return apiFetch<GraphNodeDto>('/api/playground/graph/nodes', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
}

export function createEdge(payload: CreateEdgeRequest): Promise<GraphEdgeDto> {
  return apiFetch<GraphEdgeDto>('/api/playground/graph/edges', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
}
