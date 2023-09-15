import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'apps',
      component: () => import('../views/AppsView.vue'),
    },
    {
      path: '/apps/new',
      name: 'app-new',
      component: () => import('../views/AppForm.vue'),
    },
    {
      path: '/apps/:name/edit',
      name: 'app-edit',
      component: () => import('../views/AppForm.vue'),
    },
    {
      path: '/apps/:name',
      name: 'app-home',
      component: () => import('../views/AppHome.vue'),
    }
  ],
});

export default router;
