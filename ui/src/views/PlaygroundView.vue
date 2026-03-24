<template>
  <section class="grid gap-4">
    <Card class="overflow-hidden border-border/70 bg-card/90">
      <CardHeader class="gap-5 lg:flex-row lg:items-start lg:justify-between">
        <div class="max-w-4xl space-y-3">
          <Badge variant="secondary" class="w-fit uppercase tracking-[0.18em]">
            Gephi-Lite Rebase
          </Badge>
          <CardTitle class="text-3xl leading-tight">
            Set-first graph atelier for GraphClaw
          </CardTitle>
          <CardDescription class="max-w-3xl text-sm leading-7">
            This playground borrows the structure of Gephi Lite, but the visible graph is governed
            by GraphClaw sets. The default scope is the synthetic <code>Root Set</code>; from there
            you can inspect, mutate, and turn selections into new sets.
          </CardDescription>
        </div>

        <div class="grid gap-3 md:max-w-md">
          <div class="grid grid-cols-2 gap-3">
            <label class="grid gap-1.5 text-sm text-muted-foreground">
              <span class="uppercase tracking-[0.12em] text-[0.72rem]">Root node limit</span>
              <Input v-model="nodeLimitInput" type="number" min="1" step="1" />
            </label>
            <label class="grid gap-1.5 text-sm text-muted-foreground">
              <span class="uppercase tracking-[0.12em] text-[0.72rem]">Root edge limit</span>
              <Input v-model="edgeLimitInput" type="number" min="1" step="1" />
            </label>
          </div>

          <div class="flex flex-wrap gap-2">
            <Button :disabled="isRefreshing" @click="applyRootLimits">
              Apply Root Limits
            </Button>
            <Button variant="outline" :disabled="isRefreshing" @click="refresh">
              Refresh
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent class="pt-0">
        <div class="flex flex-wrap gap-2">
          <Badge variant="outline" class="rounded-full px-3 py-1">
            {{ activeSet?.name ?? 'No active set' }}
          </Badge>
          <Badge variant="outline" class="rounded-full px-3 py-1">
            {{ resolvedSet?.nodes.length ?? 0 }} nodes / {{ resolvedSet?.edges.length ?? 0 }} edges
          </Badge>
          <Badge variant="outline" class="rounded-full px-3 py-1">
            {{ lastLoadedAt ? lastLoadedAt.toLocaleTimeString() : 'Not loaded yet' }}
          </Badge>
          <Badge variant="warning" class="rounded-full px-3 py-1" v-if="resolvedSet?.degradations.length">
            {{ resolvedSet.degradations.length }} degradation(s)
          </Badge>
        </div>
      </CardContent>
    </Card>

    <Card v-if="error" class="border-destructive/40 bg-destructive/10">
      <CardContent class="pt-6 text-sm text-destructive-foreground">
        {{ error }}
      </CardContent>
    </Card>

    <div class="flex flex-wrap items-center gap-2">
      <Button
        :variant="activeTab === 'graph' ? 'default' : 'outline'"
        @click="activeTab = 'graph'"
      >
        Graph
      </Button>
      <Button
        :variant="activeTab === 'data' ? 'default' : 'outline'"
        @click="activeTab = 'data'"
      >
        Data
      </Button>
      <div class="ml-auto min-w-[18rem]">
        <Input v-model="searchQuery" placeholder="Search nodes, edges, or sets" />
      </div>
    </div>

    <template v-if="activeTab === 'graph'">
      <div class="grid gap-4 2xl:grid-cols-[18rem_minmax(0,1fr)_24rem] xl:grid-cols-[17rem_minmax(0,1fr)]">
        <aside class="grid gap-4">
          <Card>
            <CardHeader>
              <CardTitle class="text-base">Modules</CardTitle>
              <CardDescription>Gephi-like workspace tools.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-2">
              <Button
                v-for="module in modules"
                :key="module"
                :variant="activeModule === module ? 'default' : 'outline'"
                class="justify-start"
                @click="activeModule = module"
              >
                {{ module }}
              </Button>
            </CardContent>
          </Card>

          <Card v-if="activeModule === 'overview'">
            <CardHeader>
              <CardTitle class="text-base">Overview</CardTitle>
              <CardDescription>Quick graph and set status.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-3 text-sm">
              <div class="flex items-center justify-between gap-3">
                <span class="text-muted-foreground">Active set</span>
                <span>{{ activeSet?.name ?? resolvedSet?.set_id ?? 'None' }}</span>
              </div>
              <div class="flex items-center justify-between gap-3">
                <span class="text-muted-foreground">Completeness</span>
                <span>{{ resolvedSet?.completeness ?? 'Unknown' }}</span>
              </div>
              <div class="flex items-center justify-between gap-3">
                <span class="text-muted-foreground">Selection</span>
                <span>{{ selectedNodeIds.length }} nodes</span>
              </div>
              <div class="grid gap-1.5 text-xs leading-6 text-muted-foreground">
                <p>Use the graph map for exploration and the `Data` tab for tabular inspection.</p>
                <p>The root set stays synthetic; every other set comes from the playground API.</p>
              </div>
            </CardContent>
          </Card>

          <Card v-if="activeModule === 'search'">
            <CardHeader>
              <CardTitle class="text-base">Search</CardTitle>
              <CardDescription>Find graph members and sets quickly.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-3">
              <div class="space-y-2">
                <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">Matching nodes</p>
                <button
                  v-for="node in filteredNodes.slice(0, 6)"
                  :key="node.id"
                  class="w-full rounded-lg border border-border/70 bg-background/60 px-3 py-2 text-left text-sm hover:bg-background/80"
                  @click="toggleNode(node.id)"
                >
                  {{ node.properties.name ?? `Node ${node.id}` }}
                </button>
              </div>
              <div class="space-y-2">
                <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">Matching sets</p>
                <button
                  v-for="entry in filteredSets.slice(0, 6)"
                  :key="entry.id"
                  class="w-full rounded-lg border border-border/70 bg-background/60 px-3 py-2 text-left text-sm hover:bg-background/80"
                  @click="activateSetFromTable(entry.id)"
                >
                  {{ entry.name }}
                </button>
              </div>
            </CardContent>
          </Card>

          <Card v-if="activeModule === 'layout'">
            <CardHeader>
              <CardTitle class="text-base">Layout</CardTitle>
              <CardDescription>Switch graph map projection strategies.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-2">
              <Button
                v-for="mode in layoutModes"
                :key="mode"
                :variant="layoutMode === mode ? 'default' : 'outline'"
                class="justify-start"
                @click="layoutMode = mode"
              >
                {{ mode }}
              </Button>
            </CardContent>
          </Card>

          <Card v-if="activeModule === 'appearance'">
            <CardHeader>
              <CardTitle class="text-base">Appearance</CardTitle>
              <CardDescription>Choose graph styling priorities.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-2">
              <Button
                v-for="mode in appearanceModes"
                :key="mode"
                :variant="appearanceMode === mode ? 'default' : 'outline'"
                class="justify-start"
                @click="appearanceMode = mode"
              >
                {{ mode }}
              </Button>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle class="text-base">Selection to Set</CardTitle>
              <CardDescription>Create a new governed perimeter from selected nodes.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-3">
              <Input v-model="setDraftName" placeholder="Shared business focus" />
              <textarea
                v-model="setDraftDescription"
                rows="3"
                class="rounded-md border border-input bg-background/70 px-3 py-2 text-sm outline-none ring-offset-background placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
                placeholder="Describe why this selection matters."
              />
              <select
                v-model="setDraftKind"
                class="rounded-md border border-input bg-background/70 px-3 py-2 text-sm outline-none ring-offset-background focus-visible:ring-2 focus-visible:ring-ring"
              >
                <option value="boundary">boundary</option>
                <option value="semantic">semantic</option>
                <option value="projection">projection</option>
              </select>
              <Button :disabled="selectedNodeIds.length === 0 || isRefreshing" @click="createSelectionSet">
                Create Set
              </Button>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle class="text-base">Graph Mutations</CardTitle>
              <CardDescription>Create nodes and edges directly from the playground.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-4">
              <div class="grid gap-2">
                <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">Create node</p>
                <Input v-model="nodeDraftLabels" placeholder="Concept, Business" />
                <textarea
                  v-model="nodeDraftProperties"
                  rows="3"
                  class="rounded-md border border-input bg-background/70 px-3 py-2 text-sm outline-none ring-offset-background focus-visible:ring-2 focus-visible:ring-ring"
                />
                <Button variant="outline" @click="submitNodeDraft">Create Node</Button>
              </div>

              <div class="grid gap-2">
                <p class="text-xs uppercase tracking-[0.18em] text-muted-foreground">Create edge</p>
                <div class="grid grid-cols-2 gap-2">
                  <Input v-model="edgeDraftFromId" placeholder="From id" />
                  <Input v-model="edgeDraftToId" placeholder="To id" />
                </div>
                <Input v-model="edgeDraftType" placeholder="RELATES_TO" />
                <textarea
                  v-model="edgeDraftProperties"
                  rows="2"
                  class="rounded-md border border-input bg-background/70 px-3 py-2 text-sm outline-none ring-offset-background focus-visible:ring-2 focus-visible:ring-ring"
                />
                <Button variant="outline" @click="submitEdgeDraft">Create Edge</Button>
              </div>
            </CardContent>
          </Card>
        </aside>

        <PlaygroundGraphCanvas
          :title="graphTitle"
          :nodes="resolvedSet?.nodes ?? []"
          :edges="resolvedSet?.edges ?? []"
          :is-loading="isLoading"
          :selected-node-ids="selectedNodeIds"
          :selected-edge-id="selectedEdge?.id ?? null"
          :layout-mode="layoutMode"
          :appearance-mode="appearanceMode"
          @node-toggle="toggleNode"
          @edge-click="selectEdge"
          @stage-click="clearSelection"
        />

        <PlaygroundGraphInspector
          :active-set="activeSet"
          :resolved-set="resolvedSet"
          :selected-node="selectedNode"
          :selected-edge="selectedEdge"
          :selected-node-ids="selectedNodeIds"
          :last-loaded-at="lastLoadedAt"
          :is-refreshing="isRefreshing"
        />
      </div>
    </template>

    <PlaygroundDataTables
      v-else
      :nodes="filteredNodes"
      :edges="filteredEdges"
      :sets="filteredSets"
      :selected-node-ids="selectedNodeIds"
      @toggle-node="toggleNode"
      @activate-set="activateSetFromTable"
      @select-edge="selectEdge"
    />
  </section>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import PlaygroundDataTables from '@/components/playground/PlaygroundDataTables.vue';
import PlaygroundGraphCanvas from '@/components/playground/PlaygroundGraphCanvas.vue';
import PlaygroundGraphInspector from '@/components/playground/PlaygroundGraphInspector.vue';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { usePlaygroundGraph } from '@/composables/usePlaygroundGraph';

const modules = ['overview', 'search', 'layout', 'appearance'] as const;
const layoutModes = ['grid', 'circlepack', 'radial'] as const;
const appearanceModes = ['labels', 'sets', 'density'] as const;

const {
  resolvedSet,
  selectedNodeIds,
  selectedNode,
  selectedEdge,
  error,
  isLoading,
  isRefreshing,
  activeTab,
  activeModule,
  activeSet,
  lastLoadedAt,
  nodeLimitInput,
  edgeLimitInput,
  searchQuery,
  layoutMode,
  appearanceMode,
  filteredNodes,
  filteredEdges,
  filteredSets,
  graphTitle,
  setDraftName,
  setDraftDescription,
  setDraftKind,
  nodeDraftLabels,
  nodeDraftProperties,
  edgeDraftFromId,
  edgeDraftToId,
  edgeDraftType,
  edgeDraftProperties,
  initialize,
  refresh,
  applyRootLimits,
  toggleNode,
  selectEdge,
  clearSelection,
  activateSetFromTable,
  createSelectionSet,
  submitNodeDraft,
  submitEdgeDraft,
} = usePlaygroundGraph();

onMounted(() => {
  void initialize();
});
</script>
