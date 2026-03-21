<template>
  <section class="grid gap-4">
    <Card class="overflow-hidden">
      <CardHeader class="gap-5 lg:flex-row lg:items-start lg:justify-between">
        <div class="max-w-3xl space-y-3">
          <Badge variant="secondary" class="w-fit uppercase tracking-[0.18em]">
            Gateway Playground
          </Badge>
          <CardTitle class="text-3xl leading-tight">
            Read GraphClaw through the graph
          </CardTitle>
          <CardDescription class="max-w-2xl text-sm leading-7">
            This new <code>ui/</code> surface is the seed of the future interface. Right now it
            does exactly one thing: fetch a bounded graph snapshot from the gateway and let you
            inspect nodes and edges in a Sigma canvas.
          </CardDescription>
        </div>

        <div class="grid w-full gap-3 md:max-w-sm">
          <label class="grid gap-1.5 text-sm text-muted-foreground">
            <span class="uppercase tracking-[0.12em] text-[0.72rem]">Node limit</span>
            <Input v-model="nodeLimitInput" type="number" min="1" step="1" />
          </label>

          <label class="grid gap-1.5 text-sm text-muted-foreground">
            <span class="uppercase tracking-[0.12em] text-[0.72rem]">Edge limit</span>
            <Input v-model="edgeLimitInput" type="number" min="1" step="1" />
          </label>

          <div class="flex flex-wrap gap-2 pt-1">
            <Button :disabled="isRefreshing" @click="applyLimits">
              <SlidersHorizontal class="h-4 w-4" />
              Apply
            </Button>
            <Button variant="outline" :disabled="isRefreshing" @click="refresh">
              <RefreshCcw class="h-4 w-4" />
              Refresh
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent class="pt-0">
        <div class="flex flex-wrap gap-2">
          <Badge variant="outline" class="rounded-full px-3 py-1">
            <Workflow class="mr-1.5 h-3.5 w-3.5" />
            <code>/api/playground/graph</code>
          </Badge>
          <Badge variant="outline" class="rounded-full px-3 py-1">
            <Network class="mr-1.5 h-3.5 w-3.5" />
            {{ activeNodeLimit }} nodes / {{ activeEdgeLimit }} edges
          </Badge>
          <Badge variant="outline" class="rounded-full px-3 py-1">
            <Clock3 class="mr-1.5 h-3.5 w-3.5" />
            {{ lastLoadedAt ? lastLoadedAt.toLocaleTimeString() : 'Not loaded yet' }}
          </Badge>
          <Badge
            v-if="snapshot?.meta.truncated"
            variant="warning"
            class="rounded-full px-3 py-1"
          >
            <ScanSearch class="mr-1.5 h-3.5 w-3.5" />
            Snapshot truncated by gateway limits
          </Badge>
        </div>
      </CardContent>
    </Card>

    <Card v-if="error" class="border-destructive/40 bg-destructive/10">
      <CardContent class="pt-6 text-sm text-destructive-foreground">
        {{ error }}
      </CardContent>
    </Card>

    <div class="grid gap-4 xl:grid-cols-[minmax(0,1fr)_24rem]">
      <PlaygroundGraphCanvas
        :title="graphTitle"
        :nodes="snapshot?.nodes ?? []"
        :edges="snapshot?.edges ?? []"
        :is-loading="isLoading"
        :selected-node-id="selectedNode?.id ?? null"
        :selected-edge-id="selectedEdge?.id ?? null"
        @node-click="selectNode"
        @edge-click="selectEdge"
        @stage-click="clearSelection"
      />

      <PlaygroundGraphInspector
        :selected-node="selectedNode"
        :selected-edge="selectedEdge"
        :snapshot-meta="snapshot?.meta ?? null"
        :active-node-limit="activeNodeLimit"
        :active-edge-limit="activeEdgeLimit"
        :last-loaded-at="lastLoadedAt"
        :is-refreshing="isRefreshing"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import {
  Clock3,
  Network,
  RefreshCcw,
  ScanSearch,
  SlidersHorizontal,
  Workflow,
} from 'lucide-vue-next';
import PlaygroundGraphCanvas from '@/components/playground/PlaygroundGraphCanvas.vue';
import PlaygroundGraphInspector from '@/components/playground/PlaygroundGraphInspector.vue';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { usePlaygroundGraph } from '@/composables/usePlaygroundGraph';

const {
  snapshot,
  selectedNode,
  selectedEdge,
  error,
  isLoading,
  isRefreshing,
  lastLoadedAt,
  nodeLimitInput,
  edgeLimitInput,
  activeNodeLimit,
  activeEdgeLimit,
  graphTitle,
  initialize,
  applyLimits,
  refresh,
  selectNode,
  selectEdge,
  clearSelection,
} = usePlaygroundGraph();

onMounted(() => {
  void initialize();
});
</script>
