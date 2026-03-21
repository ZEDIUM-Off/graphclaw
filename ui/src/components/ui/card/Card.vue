<script setup lang="ts">
import { computed } from 'vue';
import { cn } from '@/lib/utils';

const props = defineProps<{ class?: string }>();
const classes = computed(() =>
  cn(
    'rounded-xl border border-border bg-card/80 text-card-foreground shadow-[0_18px_60px_rgba(0,0,0,0.22)] backdrop-blur-xl',
    props.class,
  ),
);
</script>

<template>
  <div :class="classes">
    <slot />
  </div>
</template>
