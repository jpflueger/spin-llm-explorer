import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
    {
      path: '/apps',
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
