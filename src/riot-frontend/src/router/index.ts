import { createRouter, createWebHistory } from 'vue-router'
import Dashboard from '@/views/DashboardView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () =>
        Promise.resolve({
          Main: Dashboard
        })
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: () =>
        Promise.resolve({
          Main: Dashboard
        })
    }
  ]
})

export default router
