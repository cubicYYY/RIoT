<template>
  <a-config-provider
    :theme="{
      algorithm: darkModeStore.dark ? theme.darkAlgorithm : theme.defaultAlgorithm,
      token: {
        // colorPrimary: '#222222',
      }
    }"
  >
    <router-view></router-view>
  </a-config-provider>
</template>
<script lang="ts" setup>
import { theme } from 'ant-design-vue'
import { useDarkModeStore } from '@/stores/dark'
const darkModeStore = useDarkModeStore()
</script>
