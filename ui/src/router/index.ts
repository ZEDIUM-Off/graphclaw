import { createRouter, createWebHistory } from 'vue-router';
import PlaygroundView from '@/views/PlaygroundView.vue';

const router = createRouter({
  history: createWebHistory('/_ui/'),
  routes: [
    {
      path: '/',
      redirect: '/playground',
    },
    {
      path: '/playground',
      name: 'playground',
      component: PlaygroundView,
    },
  ],
});

export default router;
