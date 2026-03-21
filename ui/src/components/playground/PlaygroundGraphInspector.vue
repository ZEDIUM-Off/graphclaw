<template>
  <aside class="grid gap-4">
    <Card>
      <CardHeader>
        <CardTitle class="text-base">Snapshot</CardTitle>
        <CardDescription>Bounded gateway metadata for the current graph read.</CardDescription>
      </CardHeader>
      <CardContent class="grid gap-3 text-sm">
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Node limit</span>
          <span>{{ activeNodeLimit }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Edge limit</span>
          <span>{{ activeEdgeLimit }}</span>
        </div>
        <Separator />
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Gateway bounded</span>
          <span>{{ snapshotMeta?.truncated ? 'Yes' : 'No' }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Snapshot caps</span>
          <span class="text-right">
            {{
              snapshotMeta
                ? `${snapshotMeta.node_limit} nodes / ${snapshotMeta.edge_limit} edges`
                : 'Unavailable'
            }}
          </span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Last refresh</span>
          <span>{{ lastLoadedAt ? lastLoadedAt.toLocaleTimeString() : 'Not loaded yet' }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Status</span>
          <span>{{ isRefreshing ? 'Loading snapshot...' : 'Idle' }}</span>
        </div>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle class="text-base">Selected node</CardTitle>
      </CardHeader>
      <CardContent>
        <template v-if="selectedNode">
          <p class="mb-3 text-sm text-card-foreground">
            id={{ selectedNode.id }} labels={{ selectedNode.labels.join(', ') || 'none' }}
          </p>
          <pre
            class="overflow-auto rounded-lg border border-border/80 bg-background/70 p-3 text-xs text-muted-foreground"
          >{{ renderJson(selectedNode.properties) }}</pre>
        </template>
        <p v-else class="text-sm text-muted-foreground">No node selected.</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle class="text-base">Selected edge</CardTitle>
      </CardHeader>
      <CardContent>
        <template v-if="selectedEdge">
          <p class="mb-3 text-sm text-card-foreground">
            id={{ selectedEdge.id }} {{ selectedEdge.from_id }} -[{{ selectedEdge.rel_type }}]-&gt;
            {{ selectedEdge.to_id }}
          </p>
          <pre
            class="overflow-auto rounded-lg border border-border/80 bg-background/70 p-3 text-xs text-muted-foreground"
          >{{ renderJson(selectedEdge.properties) }}</pre>
        </template>
        <p v-else class="text-sm text-muted-foreground">No edge selected.</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle class="text-base">Gateway contract</CardTitle>
      </CardHeader>
      <CardContent class="space-y-2 text-sm text-muted-foreground">
        <p><code>GET /api/playground/graph</code> for bounded snapshots.</p>
        <p><code>GET /api/playground/graph/nodes/:id</code> for node inspection.</p>
        <p>This Vue UI is intentionally playground-only in the current migration step.</p>
      </CardContent>
    </Card>
  </aside>
</template>

<script setup lang="ts">
import type {
  GraphEdgeDto,
  GraphNodeDto,
  GraphSnapshotMetaDto,
} from '@/lib/playground';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';

defineProps<{
  selectedNode: GraphNodeDto | null;
  selectedEdge: GraphEdgeDto | null;
  snapshotMeta: GraphSnapshotMetaDto | null;
  activeNodeLimit: number;
  activeEdgeLimit: number;
  lastLoadedAt: Date | null;
  isRefreshing: boolean;
}>();

function renderJson(value: unknown): string {
  return JSON.stringify(value, null, 2);
}
</script>
