<script setup lang="ts">
import { computed } from 'vue';
import { cva } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const badgeVariants = cva(
  'inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors',
  {
    variants: {
      variant: {
        default: 'border-transparent bg-primary text-primary-foreground',
        secondary: 'border-border bg-secondary text-secondary-foreground',
        outline: 'border-border text-foreground',
        warning: 'border-amber-500/30 bg-amber-500/10 text-amber-200',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  },
);

interface BadgeProps {
  variant?: 'default' | 'secondary' | 'outline' | 'warning';
  class?: string;
}

const props = withDefaults(defineProps<BadgeProps>(), {
  variant: 'default',
});

const classes = computed(() => cn(badgeVariants({ variant: props.variant }), props.class));
</script>

<template>
  <div :class="classes">
    <slot />
  </div>
</template>
