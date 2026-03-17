import type {
  StatusResponse,
  ToolSpec,
  CronJob,
  Integration,
  DiagResult,
  MemoryEntry,
  CostSummary,
  CliTool,
  HealthSnapshot,
} from '../types/api';
import { clearToken, getToken, setToken } from './auth';

// ---------------------------------------------------------------------------
// Base fetch wrapper
// ---------------------------------------------------------------------------

export class UnauthorizedError extends Error {
  constructor() {
    super('Unauthorized');
    this.name = 'UnauthorizedError';
  }
}

export async function apiFetch<T = unknown>(
  path: string,
  options: RequestInit = {},
): Promise<T> {
  const token = getToken();
  const headers = new Headers(options.headers);

  if (token) {
    headers.set('Authorization', `Bearer ${token}`);
  }

  if (
    options.body &&
    typeof options.body === 'string' &&
    !headers.has('Content-Type')
  ) {
    headers.set('Content-Type', 'application/json');
  }

  const response = await fetch(path, { ...options, headers });

  if (response.status === 401) {
    clearToken();
    window.dispatchEvent(new Event('zeroclaw-unauthorized'));
    throw new UnauthorizedError();
  }

  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(`API ${response.status}: ${text || response.statusText}`);
  }

  // Some endpoints may return 204 No Content
  if (response.status === 204) {
    return undefined as unknown as T;
  }

  return response.json() as Promise<T>;
}

function unwrapField<T>(value: T | Record<string, T>, key: string): T {
  if (value !== null && typeof value === 'object' && !Array.isArray(value) && key in value) {
    const unwrapped = (value as Record<string, T | undefined>)[key];
    if (unwrapped !== undefined) {
      return unwrapped;
    }
  }
  return value as T;
}

// ---------------------------------------------------------------------------
// Pairing
// ---------------------------------------------------------------------------

export async function pair(code: string): Promise<{ token: string }> {
  const response = await fetch('/pair', {
    method: 'POST',
    headers: { 'X-Pairing-Code': code },
  });

  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(`Pairing failed (${response.status}): ${text || response.statusText}`);
  }

  const data = (await response.json()) as { token: string };
  setToken(data.token);
  return data;
}

// ---------------------------------------------------------------------------
// Public health (no auth required)
// ---------------------------------------------------------------------------

export async function getPublicHealth(): Promise<{ require_pairing: boolean; paired: boolean }> {
  const response = await fetch('/health');
  if (!response.ok) {
    throw new Error(`Health check failed (${response.status})`);
  }
  return response.json() as Promise<{ require_pairing: boolean; paired: boolean }>;
}

// ---------------------------------------------------------------------------
// Status / Health
// ---------------------------------------------------------------------------

export function getStatus(): Promise<StatusResponse> {
  return apiFetch<StatusResponse>('/api/status');
}

export function getHealth(): Promise<HealthSnapshot> {
  return apiFetch<HealthSnapshot | { health: HealthSnapshot }>('/api/health').then((data) =>
    unwrapField(data, 'health'),
  );
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

export function getConfig(): Promise<string> {
  return apiFetch<string | { format?: string; content: string }>('/api/config').then((data) =>
    typeof data === 'string' ? data : data.content,
  );
}

export function putConfig(toml: string): Promise<void> {
  return apiFetch<void>('/api/config', {
    method: 'PUT',
    headers: { 'Content-Type': 'application/toml' },
    body: toml,
  });
}

// ---------------------------------------------------------------------------
// Tools
// ---------------------------------------------------------------------------

export function getTools(): Promise<ToolSpec[]> {
  return apiFetch<ToolSpec[] | { tools: ToolSpec[] }>('/api/tools').then((data) =>
    unwrapField(data, 'tools'),
  );
}

// ---------------------------------------------------------------------------
// Cron
// ---------------------------------------------------------------------------

export function getCronJobs(): Promise<CronJob[]> {
  return apiFetch<CronJob[] | { jobs: CronJob[] }>('/api/cron').then((data) =>
    unwrapField(data, 'jobs'),
  );
}

export function addCronJob(body: {
  name?: string;
  command: string;
  schedule: string;
  enabled?: boolean;
}): Promise<CronJob> {
  return apiFetch<CronJob | { status: string; job: CronJob }>('/api/cron', {
    method: 'POST',
    body: JSON.stringify(body),
  }).then((data) => (typeof (data as { job?: CronJob }).job === 'object' ? (data as { job: CronJob }).job : (data as CronJob)));
}

export function deleteCronJob(id: string): Promise<void> {
  return apiFetch<void>(`/api/cron/${encodeURIComponent(id)}`, {
    method: 'DELETE',
  });
}

// ---------------------------------------------------------------------------
// Integrations
// ---------------------------------------------------------------------------

export function getIntegrations(): Promise<Integration[]> {
  return apiFetch<Integration[] | { integrations: Integration[] }>('/api/integrations').then(
    (data) => unwrapField(data, 'integrations'),
  );
}

// ---------------------------------------------------------------------------
// Doctor / Diagnostics
// ---------------------------------------------------------------------------

export function runDoctor(): Promise<DiagResult[]> {
  return apiFetch<DiagResult[] | { results: DiagResult[]; summary?: unknown }>('/api/doctor', {
    method: 'POST',
    body: JSON.stringify({}),
  }).then((data) => (Array.isArray(data) ? data : data.results));
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

export function getMemory(
  query?: string,
  category?: string,
): Promise<MemoryEntry[]> {
  const params = new URLSearchParams();
  if (query) params.set('query', query);
  if (category) params.set('category', category);
  const qs = params.toString();
  return apiFetch<MemoryEntry[] | { entries: MemoryEntry[] }>(`/api/memory${qs ? `?${qs}` : ''}`).then(
    (data) => unwrapField(data, 'entries'),
  );
}

export function storeMemory(
  key: string,
  content: string,
  category?: string,
): Promise<void> {
  return apiFetch<unknown>('/api/memory', {
    method: 'POST',
    body: JSON.stringify({ key, content, category }),
  }).then(() => undefined);
}

export function deleteMemory(key: string): Promise<void> {
  return apiFetch<void>(`/api/memory/${encodeURIComponent(key)}`, {
    method: 'DELETE',
  });
}

// ---------------------------------------------------------------------------
// Cost
// ---------------------------------------------------------------------------

export function getCost(): Promise<CostSummary> {
  return apiFetch<CostSummary | { cost: CostSummary }>('/api/cost').then((data) =>
    unwrapField(data, 'cost'),
  );
}

// ---------------------------------------------------------------------------
// CLI Tools
// ---------------------------------------------------------------------------

export function getCliTools(): Promise<CliTool[]> {
  return apiFetch<CliTool[] | { cli_tools: CliTool[] }>('/api/cli-tools').then((data) =>
    unwrapField(data, 'cli_tools'),
  );
}

// ---------------------------------------------------------------------------
// GraphClaw Playground
// ---------------------------------------------------------------------------

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

export interface SubgraphDto {
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
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

export interface ViewTemplateDto {
  id: string;
  name: string;
  kind: string;
  description: string;
  extends?: string[];
  selectors?: { node_ids?: number[]; label?: string; props?: Record<string, unknown> };
  filters?: string[];
  operations?: ViewOperationDto[];
  cost_limit?: number;
}

export type ViewOperationDto =
  | { op: 'union'; view_ids: string[] }
  | { op: 'intersection'; view_ids: string[] }
  | { op: 'difference'; a: string; b: string }
  | { op: 'expand'; relation_type?: string; depth: number }
  | { op: 'filter_nodes'; predicate: string }
  | { op: 'filter_edges'; predicate: string }
  | { op: 'project'; mode: string }
  | { op: 'slice'; limit: number; order?: string };

export interface ResolvedViewDto {
  view_id: string;
  view_kind?: string;
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  composition_trace?: string[];
  completeness?: string;
  degradations?: string[];
  cost_estimate?: number;
}

export interface ResolvedViewExportDto {
  format: string;
  structured: Record<string, unknown>;
  text: string;
}

export function playgroundCreateNode(labels: string[], properties: Record<string, unknown>): Promise<GraphNodeDto> {
  return apiFetch<GraphNodeDto>('/api/playground/graph/nodes', {
    method: 'POST',
    body: JSON.stringify({ labels, properties }),
  });
}

export function playgroundCreateEdge(
  from_id: number,
  to_id: number,
  rel_type: string,
  properties: Record<string, unknown> = {},
): Promise<GraphEdgeDto> {
  return apiFetch<GraphEdgeDto>('/api/playground/graph/edges', {
    method: 'POST',
    body: JSON.stringify({ from_id, to_id, rel_type, properties }),
  });
}

export function playgroundGetSubgraph(node_ids: number[]): Promise<SubgraphDto> {
  return apiFetch<SubgraphDto>('/api/playground/graph/subgraph', {
    method: 'POST',
    body: JSON.stringify({ node_ids }),
  });
}

export function playgroundGetGraph(
  options: { node_limit?: number; edge_limit?: number } = {},
): Promise<GraphSnapshotDto> {
  const params = new URLSearchParams();
  if (typeof options.node_limit === 'number') {
    params.set('node_limit', String(options.node_limit));
  }
  if (typeof options.edge_limit === 'number') {
    params.set('edge_limit', String(options.edge_limit));
  }
  const suffix = params.toString();
  return apiFetch<GraphSnapshotDto>(`/api/playground/graph${suffix ? `?${suffix}` : ''}`);
}

export function playgroundInspectNode(id: number): Promise<GraphNodeDto | null> {
  return apiFetch<GraphNodeDto | { error: string }>(`/api/playground/graph/nodes/${id}`).then((data) => {
    if (data && typeof data === 'object' && 'error' in data) return null;
    return data as GraphNodeDto;
  });
}

export function playgroundListViews(): Promise<string[]> {
  return apiFetch<string[]>('/api/playground/views');
}

export function playgroundGetView(id: string): Promise<ViewTemplateDto> {
  return apiFetch<ViewTemplateDto>(`/api/playground/views/${encodeURIComponent(id)}`);
}

export function playgroundCreateView(template: ViewTemplateDto): Promise<{ ok: boolean }> {
  return apiFetch<{ ok: boolean }>('/api/playground/views', {
    method: 'POST',
    body: JSON.stringify(template),
  });
}

export function playgroundUpdateView(id: string, template: ViewTemplateDto): Promise<{ ok: boolean }> {
  return apiFetch<{ ok: boolean }>(`/api/playground/views/${encodeURIComponent(id)}`, {
    method: 'PUT',
    body: JSON.stringify(template),
  });
}

export function playgroundBind(template_id: string, anchors: Record<string, unknown>): Promise<unknown> {
  return apiFetch<unknown>('/api/playground/views/bind', {
    method: 'POST',
    body: JSON.stringify({ template_id, anchors, parameters: {}, resolution_scope: null }),
  });
}

export function playgroundResolve(template_id: string, anchors: Record<string, unknown>): Promise<ResolvedViewDto> {
  return apiFetch<ResolvedViewDto>('/api/playground/views/resolve', {
    method: 'POST',
    body: JSON.stringify({ template_id, anchors, parameters: {}, resolution_scope: null }),
  });
}

export function playgroundExport(
  resolved: ResolvedViewDto,
  format: 'llm_compact' | 'llm_explained',
  options?: { purpose?: string; constraints?: string[]; usage_hint?: string; role?: string; included?: string[]; excluded?: string[] },
): Promise<ResolvedViewExportDto> {
  return apiFetch<ResolvedViewExportDto>('/api/playground/views/export', {
    method: 'POST',
    body: JSON.stringify({
      format,
      resolved,
      purpose: options?.purpose,
      constraints: options?.constraints ?? [],
      usage_hint: options?.usage_hint,
      role: options?.role,
      included: options?.included ?? [],
      excluded: options?.excluded,
    }),
  });
}
