<template>
  <a-config-provider
    :theme="{
      algorithm: darkModeStore.dark ? theme.darkAlgorithm : theme.defaultAlgorithm,
      token: {
        // colorPrimary: '#222222',
      }
    }"
  >
    <Suspense>
      <router-view></router-view>
      <template #fallback> <a-spin size="large" /> </template>
    </Suspense>
  </a-config-provider>
</template>
<script lang="ts" setup>
import { theme } from 'ant-design-vue'
import { useDarkModeStore } from '@/stores/dark'
const darkModeStore = useDarkModeStore()
</script>
