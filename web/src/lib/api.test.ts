import { beforeEach, describe, expect, it, vi } from 'vitest';
import { getToken, setToken } from './auth';
import {
  UnauthorizedError,
  apiFetch,
  playgroundExport,
  playgroundInspectNode,
  type ResolvedViewDto,
} from './api';

describe('api helpers', () => {
  const fetchMock = vi.fn<typeof fetch>();

  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal('fetch', fetchMock);
    localStorage.clear();
  });

  it('adds auth and json headers when sending a string body', async () => {
    setToken('secret-token');
    fetchMock.mockResolvedValue(
      new Response(JSON.stringify({ ok: true }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      }),
    );

    const result = await apiFetch<{ ok: boolean }>('/api/example', {
      method: 'POST',
      body: JSON.stringify({ hello: 'world' }),
    });

    expect(result.ok).toBe(true);

    const [, init] = fetchMock.mock.calls[0] ?? [];
    const headers = new Headers(init?.headers);
    expect(headers.get('authorization')).toBe('Bearer secret-token');
    expect(headers.get('content-type')).toBe('application/json');
  });

  it('clears the token and emits an event on unauthorized responses', async () => {
    setToken('expired-token');
    const listener = vi.fn<(event: Event) => void>();
    window.addEventListener('zeroclaw-unauthorized', listener);
    fetchMock.mockResolvedValue(new Response('', { status: 401, statusText: 'Unauthorized' }));

    await expect(apiFetch('/api/protected')).rejects.toBeInstanceOf(UnauthorizedError);

    expect(getToken()).toBeNull();
    expect(listener).toHaveBeenCalledTimes(1);
    window.removeEventListener('zeroclaw-unauthorized', listener);
  });

  it('maps not-found payloads to null in playgroundInspectNode', async () => {
    fetchMock.mockResolvedValue(
      new Response(JSON.stringify({ error: 'node not found' }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      }),
    );

    await expect(playgroundInspectNode(42)).resolves.toBeNull();
  });

  it('sends default export arrays when optional fields are omitted', async () => {
    const resolved: ResolvedViewDto = {
      view_id: 'view_business',
      view_kind: 'semantic',
      nodes: [],
      edges: [],
      composition_trace: [],
      degradations: [],
      cost_estimate: 0,
    };
    fetchMock.mockResolvedValue(
      new Response(JSON.stringify({ format: 'llm_compact', structured: {}, text: 'ok' }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
      }),
    );

    await playgroundExport(resolved, 'llm_compact', { purpose: 'reasoning only' });

    const [, init] = fetchMock.mock.calls[0] ?? [];
    const payload = JSON.parse(String(init?.body)) as {
      purpose?: string;
      constraints: string[];
      included: string[];
      excluded?: string[];
    };

    expect(payload.purpose).toBe('reasoning only');
    expect(payload.constraints).toEqual([]);
    expect(payload.included).toEqual([]);
    expect(payload.excluded).toBeUndefined();
  });
});
