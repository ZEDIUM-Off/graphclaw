<template>
  <div class="min-h-screen p-4 md:p-6">
    <header
      class="mx-auto flex max-w-7xl flex-col gap-4 rounded-3xl border border-border/70 bg-card/70 p-4 shadow-[0_18px_60px_rgba(0,0,0,0.26)] backdrop-blur-xl md:flex-row md:items-center md:justify-between"
    >
      <div class="flex items-center gap-4">
        <div
          class="grid h-12 w-12 place-items-center rounded-2xl bg-gradient-to-br from-primary to-accent text-sm font-black tracking-[0.18em] text-primary-foreground"
        >
          GC
        </div>
        <div>
          <p class="text-[0.72rem] uppercase tracking-[0.22em] text-accent">Future Interface</p>
          <h1 class="text-lg font-semibold tracking-tight">GraphClaw UI</h1>
        </div>
      </div>

      <nav class="flex items-center gap-2">
        <RouterLink
          to="/playground"
          class="inline-flex items-center rounded-full border border-primary/25 bg-primary/10 px-4 py-2 text-sm font-medium text-primary transition-colors hover:bg-primary/15"
        >
          Playground
        </RouterLink>
      </nav>
    </header>

    <main class="mx-auto mt-4 max-w-7xl">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router';
</script>
