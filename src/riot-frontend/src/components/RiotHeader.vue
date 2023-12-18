<template>
  <a-layout-header :style="headerStyle">
    <a-button type="primary" style="margin: 16px;" @click="isCollapsed.toggle()">
      <MenuUnfoldOutlined v-if="isCollapsed.collapsed" class="trigger" />
      <MenuFoldOutlined v-else class="trigger" />
    </a-button>
  </a-layout-header>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue';
import { useCollapseStore } from './RiotSidebar.vue';
import {
  MenuUnfoldOutlined,
  MenuFoldOutlined,
} from '@ant-design/icons-vue';
const headerStyle: CSSProperties = {
  height: '8%',
  alignItems: 'center',
  display: 'flex',
  justifyContent: 'space-between',
  position: 'sticky',
  top: 0,
  right: 0,
  paddingLeft: 0,
  backgroundColor: '#7dbcea',
};
const isCollapsed = useCollapseStore();

</script>
  