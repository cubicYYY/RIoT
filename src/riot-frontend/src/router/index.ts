const DashboardView = () => import('@/views/DashboardView.vue')
const DeviceSubView = () => import('@/views/device/DeviceSubView.vue')
const DeviceView = () => import('@/views/device/DeviceView.vue')
const LoginView = () => import('@/views/LoginPage.vue')
const RegisterView = () => import('@/views/RegisterPage.vue')
const SiteStatisticSubView = () => import('@/views/SiteStatisticSubView.vue')
const TagSubView = () => import('@/views/tag/TagSubView.vue')
const UserSubView = () => import('@/views/UserSubView.vue')
const PageNotFound = () => import('@/views/httpStatus/PageNotFound.vue')
const TypeSubView = () => import('@/views/type/TypeSubView.vue')
const RecordDetailView = () => import('@/views/device/RecordDetailSubView.vue')
import { useUserStore } from '@/stores/user'
import message from 'ant-design-vue/es/message'
import { createRouter, createWebHistory } from 'vue-router'
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/dashboard',
      name: 'dashboard',
      component: DashboardView,
      meta: { title: '仪表盘主页', keepAlive: true },
      children: [
        {
          path: 'stat',
          name: 'stat',
          component: SiteStatisticSubView,
          meta: { title: '站点统计' },
          alias: ''
        },
        {
          path: 'device',
          component: DeviceView,
          meta: { title: '设备' },
          children: [
            {
              path: '',
              component: DeviceSubView,
              meta: { title: '设备列表' }
            },
            {
              path: ':id',
              name: 'data',
              component: RecordDetailView,
              meta: { title: '设备数据' }
            }
          ]
        },
        {
          path: 'type',
          name: 'type',
          component: TypeSubView,
          meta: { title: '数据格式配置' }
        },
        {
          path: 'tag',
          name: 'tag',
          component: TagSubView,
          meta: { title: '标签管理' }
        },
        {
          path: 'user',
          name: 'user',
          component: UserSubView,
          meta: { title: '账号设置' }
        }
      ]
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView,
      meta: { title: '登录' }
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
      meta: { title: '注册' }
    },
    { path: '/:catchAll(.*)', component: PageNotFound }
  ]
})
router.beforeEach(async (to, from) => {
  const userStore = useUserStore()
  if (!userStore.inited()) {
    if (!(await userStore.init())) {
      message.error('无法连接后端服务器：请检查网络连接')
    }
  }
  if (to.path === '/') {
    console.log(userStore)
    if (userStore.loggedIn()) {
      return { path: '/dashboard' }
    } else {
      return { path: '/login' }
    }
  }
  if (!userStore.loggedIn() && to.name !== 'login' && to.name !== 'register') {
    return { name: 'login' }
  }
  return true
})
export default router
