const DashboardView = () => import ('@/views/DashboardView.vue')
const DeviceSubView = () => import ('@/views/device/DeviceSubView.vue')
const DeviceView = () => import ('@/views/device/DeviceView.vue')
const LoginView = () => import ('@/views/LoginPage.vue')
const RegisterView = () => import ('@/views/RegisterPage.vue')
const SiteStatisticSubView = () => import ('@/views/SiteStatisticSubView.vue')
const TagSubView = () => import ('@/views/tag/TagSubView.vue')
const UserSubView = () => import ('@/views/UserSubView.vue')
const PageNotFound = () => import ('@/views/httpStatus/PageNotFound.vue')
const TypeSubView = () => import ('@/views/type/TypeSubView.vue')
const RecordDetailView = () => import ('@/views/device/RecordDetailSubView.vue')
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: to => {
        // eslint-disable-next-line no-constant-condition
        if (/* isLoggedIn */true) {
          return { path: '/dashboard'}
        } else {
          return { path: '/login'}
        }
        
      },
    },
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
          component: DeviceView,
          meta: { title: '设备'},
          children: [
            {
              path: '',
              component: DeviceSubView,
              meta: { title: '设备列表'},
            },
            {
              path: ':id',
              component: RecordDetailView,
              meta: { title: '设备数据'},
            }
          ],
        },
        {
          path: 'type',
          component: TypeSubView,
          meta: { title: '数据格式配置'},
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
      component: LoginView,
      meta: { title: '登录' },
    },
    {
      path: '/register',
      component: RegisterView,
      meta: { title: '注册' },
    },
    { path: "/:catchAll(.*)", component: PageNotFound }
  ]
})

export default router
