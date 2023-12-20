<template>
  <a-layout-header :style="headerStyle">
    <!-- Collapse Button -->
    <a-tooltip title="收起/展开 侧边栏">
      <a-button type="dashed" style="margin-left: 16px;" @click="isCollapsed.toggle()">
        <MenuUnfoldOutlined v-if="isCollapsed.collapsed" class="trigger" />
        <MenuFoldOutlined v-else class="trigger" />
      </a-button>
    </a-tooltip>
    <a-flex style="flex:1;" />
    <!-- Dark Mode Switch -->
    <a-tooltip title="黑暗/日间 模式切换">
      <a-button type="default" shape="round" style="margin-left: auto;" @click="isDarkMode.toggle()">
        <DarkMoon v-if="isDarkMode.dark" class="trigger" />
        <LightSun v-else class="trigger" />
      </a-button>
    </a-tooltip>
    <a-menu mode="horizontal" style="margin-left: 16px;">
      <a-menu-item key="5">
        <a-tooltip title="账号设置">
          <router-link to="/dashboard/user">
            <UserOutlined class="trigger" style="margin: 8px;" />{{ props.username }}
          </router-link>
        </a-tooltip>
      </a-menu-item>
    </a-menu>
  </a-layout-header>
</template>
<script lang="ts" setup>
const props = defineProps(['username'])
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
  zIndex: 1,
};
const isCollapsed = useCollapseStore();
const isDarkMode = useDarkModeStore();

</script>
  