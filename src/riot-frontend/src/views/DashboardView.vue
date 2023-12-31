<template>
  <a-space direction="vertical" :style="{ width: '100%' }">
    <a-layout :style="appStyle" has-sider>
      <RiotSidebar />
      <a-layout :style="rightLayoutStyle">
        <RiotHeader :username="userStore.data?.username" />
        <a-breadcrumb :routes="breadcrumpRoutes" style="margin: 16px">
          <template #itemRender="{ route, paths }">
            <span v-if="breadcrumpRoutes.indexOf(route) === breadcrumpRoutes.length - 1">
              {{ route.breadcrumbName }}
            </span>
            <router-link v-else :to="`/${paths[breadcrumpRoutes.indexOf(route)]}`">
              {{ route.breadcrumbName }}
            </router-link>
          </template>
        </a-breadcrumb>
        <a-layout-content :style="contentStyle">
          <router-view v-slot="{ Component }">
            <Transition name="subview-fade" appear>
              <Suspense>
                <div>
                  <component :is="Component" />
                </div>
              </Suspense>
            </Transition>
          </router-view>
        </a-layout-content>
      </a-layout>
    </a-layout>
  </a-space>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue'
import { ref, watch } from 'vue'
import RiotHeader from '../components/RiotHeader.vue'
import RiotSidebar from '../components/RiotSidebar.vue'
import { useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
const userStore = useUserStore()
interface Route {
  path: string
  breadcrumbName: string
  children?: Array<{
    path: string
    breadcrumbName: string
  }>
}
declare module 'vue-router' {
  interface RouteMeta {
    title: string
  }
}
// Automatically derived breadcrumb
function routeDeduplicate(routes: Route[]) {
  const filteredList: Route[] = []
  for (let i = 0; i < routes.length; i++) {
    // Skip the first item, as there is no previous item to compare
    if (i === 0) {
      filteredList.push(routes[i])
      continue
    }

    // Compare the current item's path with the previous item's path
    if (routes[i].path !== routes[i - 1].path) {
      filteredList.push(routes[i])
    }
  }

  return filteredList
}
const route = useRoute()
const breadcrumpRoutes = ref(
  routeDeduplicate(
    route.matched.map((r) => ({ breadcrumbName: r.meta.title, path: r.path }) as Route)
  )
)
watch(
  () => route.fullPath,
  async () => {
    if (typeof breadcrumpRoutes.value === 'undefined') return
    breadcrumpRoutes.value = routeDeduplicate(
      route.matched.map((r) => ({ breadcrumbName: r.meta.title, path: r.path }) as Route)
    )
  },
  {
    immediate: true
  }
)
const appStyle: CSSProperties = {
  display: 'flex',
  minHeight: '100vh',
  width: '100%'
}

const rightLayoutStyle: CSSProperties = {
  display: 'flex',
  flex: 1,
  flexDirection: 'column'
}

const contentStyle: CSSProperties = {
  textAlign: 'center',
  lineHeight: '32px',
  padding: '24px',
  color: '#aaa',
  overflow: 'clip',
  margin: '0 16px',
  flex: 1
}
</script>
<style scoped>
/* Removed to avoid overlapping of the old and the new subview */
/* .subview-fade-leave-active, */
.subview-fade-enter-active {
  transition: opacity 0.3s ease;
}

.subview-fade-enter-from,
.subview-fade-leave-to {
  opacity: 0;
}
</style>
