<template>
  <Suspense>
    <a-config-provider :theme="{
      algorithm: darkModeStore.dark ? theme.darkAlgorithm : theme.defaultAlgorithm,
      token: {
        // colorPrimary: '#222222',
      },
    }" apiRoot="http://127.0.0.1:8888/api">
      <router-view></router-view>
    </a-config-provider>
    <template #fallback>
      Load failed.
    </template>
  </Suspense>
</template>
<script lang="ts" setup>
import { theme } from 'ant-design-vue';
import { useDarkModeStore } from '@/stores/dark';
const darkModeStore = useDarkModeStore()
</script>