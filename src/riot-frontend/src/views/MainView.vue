<template>
  <a-space direction="vertical" :style="{ width: '100%' }">
    <a-layout :style="appStyle">
      <a-layout-sider :style="sidebarStyle">Sider</a-layout-sider>
      <a-layout :style="mainLayoutStyle">
        <a-layout-header :style="headerStyle">RIoT</a-layout-header>
        <a-layout-content :style="contentStyle">{{ content }}</a-layout-content>
      </a-layout>
    </a-layout>
  </a-space>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue';
let content = "a\n".repeat(1000);

const appStyle: CSSProperties = {
  width: '100%',
  height: '100%',
  display: 'flex',
};

const mainLayoutStyle: CSSProperties = {
  display: 'flex',
  flex: 1,
  flexDirection: 'column',
};

const headerStyle: CSSProperties = {
  height: '8%',
  alignItems: 'center',
  display: 'flex',
  justifyContent: 'space-between',
  position: 'sticky',
  top: 0,
  right: 0,
  backgroundColor: '#7dbcea',
};

const contentStyle: CSSProperties = {
  textAlign: 'center',
  minHeight: '120px',
  lineHeight: '32px',
  padding: '12px',
  color: '#fff',
  backgroundColor: '#108ee9',
  overflow: 'clip',
  flex: 1,
};

const sidebarStyle: CSSProperties = {
  textAlign: 'center',
  lineHeight: '120px',
  color: '#fff',
  backgroundColor: '#3ba0e9',
  overflow: 'auto',
  height: '100vh',
  position: 'sticky',
  top: 0,
};

</script>
  