const DashboardView = () => import ('@/views/DashboardView.vue')
const DeviceSubView = () => import ('@/views/DeviceSubView.vue')
const LoginOrRegisterView = () => import ('@/views/LoginOrRegisterView.vue')
const SiteStatisticSubView = () => import ('@/views/SiteStatisticSubView.vue')
const TagSubView = () => import ('@/views/TagSubView.vue')
const UserSubView = () => import ('@/views/UserSubView.vue')
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/dashboard',
      component: DashboardView,
      meta: { title: '仪表盘主页', keepAlive: true},
      children: [
        {
          path: 'stat',
          component: SiteStatisticSubView,
          meta: { title: '站点统计'},
          alias: '',
        },
        {
          path: 'device',
          component: DeviceSubView,
          meta: { title: '设备管理'},
        },
        {
          path: 'tag',
          component: TagSubView,
          meta: { title: '标签管理'},
        },
        {
          path: 'user',
          component: UserSubView,
          meta: { title: '账号设置'},
        },
      ]
    },
    {
      path: '/login',
      component: LoginOrRegisterView,
      meta: { title: '登录' },
      alias: '/register'
    },
  ]
})

export default router
