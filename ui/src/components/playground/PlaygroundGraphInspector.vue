<template>
  <aside class="grid gap-4">
    <Card>
      <CardHeader>
        <CardTitle class="text-base">Active Set</CardTitle>
        <CardDescription>The graph map is currently derived from this set.</CardDescription>
      </CardHeader>
      <CardContent class="grid gap-3 text-sm">
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Name</span>
          <span class="text-right font-medium">{{ activeSet?.name ?? resolvedSet?.set_id ?? 'None' }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Kind</span>
          <span>{{ activeSet?.kind ?? resolvedSet?.set_kind ?? 'Unknown' }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Completeness</span>
          <span>{{ resolvedSet?.completeness ?? 'Unavailable' }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Last refresh</span>
          <span>{{ lastLoadedAt ? lastLoadedAt.toLocaleTimeString() : 'Not loaded yet' }}</span>
        </div>
        <div class="flex items-center justify-between gap-3">
          <span class="text-muted-foreground">Cost estimate</span>
          <span>{{ resolvedSet?.cost_estimate ?? 'Unknown' }}</span>
        </div>
        <Separator />
        <p class="text-xs leading-6 text-muted-foreground">
          {{ activeSet?.description || 'Synthetic root set exposing the widest playground scope.' }}
        </p>
        <p
          v-if="resolvedSet?.degradations.length"
          class="rounded-lg border border-amber-300/20 bg-amber-300/10 p-3 text-xs leading-6 text-amber-100"
        >
          {{ resolvedSet.degradations.join(' · ') }}
        </p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle class="text-base">Selection</CardTitle>
        <CardDescription>Current graph focus.</CardDescription>
      </CardHeader>
      <CardContent class="grid gap-3">
        <p class="text-sm text-muted-foreground">{{ selectedNodeIds.length }} selected nodes</p>
        <p class="text-xs leading-6 text-muted-foreground">
          Build a Set from the current selection in the left module rail, then activate it from the
          `Sets` table.
        </p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle class="text-base">Selected Node</CardTitle>
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
        <CardTitle class="text-base">Selected Edge</CardTitle>
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
        <CardTitle class="text-base">Composition Trace</CardTitle>
      </CardHeader>
      <CardContent class="space-y-2">
        <p
          v-for="step in resolvedSet?.composition_trace ?? []"
          :key="step"
          class="rounded-md border border-border/70 bg-background/60 px-3 py-2 text-xs text-muted-foreground"
        >
          {{ step }}
        </p>
        <p v-if="!(resolvedSet?.composition_trace.length)" class="text-sm text-muted-foreground">
          No trace available for this set yet.
        </p>
      </CardContent>
    </Card>
  </aside>
</template>

<script setup lang="ts">
import type { GraphEdgeDto, GraphNodeDto, ResolvedSetDto, SetDefinitionDto } from '@/lib/playground';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';

defineProps<{
  activeSet: SetDefinitionDto | null;
  resolvedSet: ResolvedSetDto | null;
  selectedNode: GraphNodeDto | null;
  selectedEdge: GraphEdgeDto | null;
  selectedNodeIds: number[];
  lastLoadedAt: Date | null;
  isRefreshing: boolean;
}>();

function renderJson(value: unknown): string {
  return JSON.stringify(value, null, 2);
}
</script>
