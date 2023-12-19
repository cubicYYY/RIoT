<template>
  <a-config-provider :theme="{
    algorithm: theme.darkAlgorithm,
    token: {
      // colorPrimary: '#222222',
    },
  }">
    <a-space direction="vertical" :style="{ width: '100%' }">
      <a-layout :style="appStyle">
        <RiotSidebar />
        <a-layout :style="rightLayoutStyle">
          <RiotHeader />
          <a-layout-content :style="contentStyle">
            <router-view></router-view>
          </a-layout-content>
        </a-layout>
      </a-layout>
    </a-space>
  </a-config-provider>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue';
import { theme } from 'ant-design-vue';
import RiotHeader from '../components/RiotHeader.vue'
import RiotSidebar from '../components/RiotSidebar.vue'
const appStyle: CSSProperties = {
  display: 'flex',
  minHeight: '100vh',
  width: '100%',
};

const rightLayoutStyle: CSSProperties = {
  display: 'flex',
  flex: 1,
  flexDirection: 'column',
};

const contentStyle: CSSProperties = {
  textAlign: 'center',
  lineHeight: '32px',
  padding: '12px',
  color: '#fff',
  overflow: 'clip',
  flex: 1,
};

</script>
  