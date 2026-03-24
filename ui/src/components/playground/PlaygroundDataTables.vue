<template>
  <section class="grid gap-4 xl:grid-cols-[minmax(0,1.2fr)_minmax(0,1fr)]">
    <Card class="overflow-hidden">
      <CardHeader class="border-b border-border/70">
        <CardTitle class="text-base">Nodes</CardTitle>
        <CardDescription>Active set members rendered as a data workspace.</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="overflow-x-auto">
          <table class="min-w-full text-left text-sm">
            <thead class="bg-background/70 text-xs uppercase tracking-[0.18em] text-muted-foreground">
              <tr>
                <th class="px-4 py-3">Select</th>
                <th class="px-4 py-3">Id</th>
                <th class="px-4 py-3">Labels</th>
                <th class="px-4 py-3">Name</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="node in nodes"
                :key="node.id"
                class="border-t border-border/60 hover:bg-background/50"
              >
                <td class="px-4 py-3">
                  <input
                    type="checkbox"
                    :checked="selectedNodeIds.includes(node.id)"
                    @change="$emit('toggleNode', node.id)"
                  />
                </td>
                <td class="px-4 py-3 font-mono text-xs">{{ node.id }}</td>
                <td class="px-4 py-3">{{ node.labels.join(', ') || 'none' }}</td>
                <td class="px-4 py-3">{{ node.properties.name ?? 'Unnamed node' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </CardContent>
    </Card>

    <Card class="overflow-hidden">
      <CardHeader class="border-b border-border/70">
        <CardTitle class="text-base">Sets</CardTitle>
        <CardDescription>Persisted graph perimeters and the synthetic root set.</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="overflow-x-auto">
          <table class="min-w-full text-left text-sm">
            <thead class="bg-background/70 text-xs uppercase tracking-[0.18em] text-muted-foreground">
              <tr>
                <th class="px-4 py-3">Id</th>
                <th class="px-4 py-3">Kind</th>
                <th class="px-4 py-3">Name</th>
                <th class="px-4 py-3 text-right">Action</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="entry in sets"
                :key="entry.id"
                class="border-t border-border/60 hover:bg-background/50"
              >
                <td class="px-4 py-3 font-mono text-xs">{{ entry.id }}</td>
                <td class="px-4 py-3">{{ entry.kind }}</td>
                <td class="px-4 py-3">
                  <p>{{ entry.name }}</p>
                  <p class="text-xs text-muted-foreground">{{ entry.description }}</p>
                </td>
                <td class="px-4 py-3 text-right">
                  <Button size="sm" variant="outline" @click="$emit('activateSet', entry.id)">
                    Activate
                  </Button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </CardContent>
    </Card>

    <Card class="overflow-hidden xl:col-span-2">
      <CardHeader class="border-b border-border/70">
        <CardTitle class="text-base">Edges</CardTitle>
        <CardDescription>Relationships visible inside the active set.</CardDescription>
      </CardHeader>
      <CardContent class="p-0">
        <div class="overflow-x-auto">
          <table class="min-w-full text-left text-sm">
            <thead class="bg-background/70 text-xs uppercase tracking-[0.18em] text-muted-foreground">
              <tr>
                <th class="px-4 py-3">Id</th>
                <th class="px-4 py-3">From</th>
                <th class="px-4 py-3">Type</th>
                <th class="px-4 py-3">To</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="edge in edges"
                :key="edge.id"
                class="border-t border-border/60 hover:bg-background/50"
                @click="$emit('selectEdge', edge.id)"
              >
                <td class="px-4 py-3 font-mono text-xs">{{ edge.id }}</td>
                <td class="px-4 py-3">{{ edge.from_id }}</td>
                <td class="px-4 py-3">{{ edge.rel_type }}</td>
                <td class="px-4 py-3">{{ edge.to_id }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </CardContent>
    </Card>
  </section>
</template>

<script setup lang="ts">
import type { GraphEdgeDto, GraphNodeDto, SetDefinitionDto } from '@/lib/playground';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

defineProps<{
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  sets: SetDefinitionDto[];
  selectedNodeIds: number[];
}>();

defineEmits<{
  toggleNode: [nodeId: number];
  activateSet: [setId: string];
  selectEdge: [edgeId: number];
}>();
</script>
