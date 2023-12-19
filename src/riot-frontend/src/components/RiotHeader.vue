<template>
  <a-layout-header :style="headerStyle">
    <!-- Collapse Button -->
    <a-button type="dashed" style="margin-left: 16px;" @click="isCollapsed.toggle()">
      <MenuUnfoldOutlined v-if="isCollapsed.collapsed" class="trigger" />
      <MenuFoldOutlined v-else class="trigger" />
    </a-button>
    <a-flex style="flex:1;" />
    <!-- Dark Mode Switch -->
    <a-button type="default" shape="round" style="margin-left: auto;" @click="isDarkMode.toggle()">
      <DarkMoon v-if="isDarkMode.dark" class="trigger" />
      <LightSun v-else class="trigger" />
    </a-button>
    <a-menu mode="horizontal" style="margin-left: 16px;">
      <a-menu-item key="5">
        <router-link to="/user">
          <UserOutlined class="trigger" style="margin: 8px;"/>账号设置
        </router-link>
      </a-menu-item>
    </a-menu>
  </a-layout-header>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue';
import { useCollapseStore } from './RiotSidebar.vue';
import {
  MenuUnfoldOutlined,
  MenuFoldOutlined,
  UserOutlined,
} from '@ant-design/icons-vue';
import { useDarkModeStore } from '@/stores/dark';
import DarkMoon from './DarkMoon.vue';
import LightSun from './LightSun.vue';
const headerStyle: CSSProperties = {
  height: '64px',
  alignItems: 'center',
  display: 'flex',
  position: 'sticky',
  top: 0,
  right: 0,
  paddingLeft: 0,
  paddingRight: 0,
  overflow: 'clip',
};
const isCollapsed = useCollapseStore();
const isDarkMode = useDarkModeStore();

</script>
<style></style>
  