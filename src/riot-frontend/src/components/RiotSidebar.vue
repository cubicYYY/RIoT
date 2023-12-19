<template>
    <a-layout-sider :trigger="null" v-model:collapsed="useCollapseStore().collapsed" collapsible>
        <a-flex :style="siderStyle" vertical>
            <Slogan id="slogan" />
            <a-menu id="side-bar-menu-main" mode="inline" theme="light" :inlineCollapsed="useCollapseStore().collapsed"
                :style="sidebarMenuStyle" class="ant-menu-inline" :items="navItems">
            </a-menu>
        </a-flex>
    </a-layout-sider>
</template>

<script lang="ts" setup>
import { h, reactive, type CSSProperties } from 'vue';
import Slogan from './RiotSlogan.vue';
import {
    DashboardOutlined,
    DatabaseOutlined,
    TagsOutlined,
    BookOutlined,
} from '@ant-design/icons-vue';
import { defineStore } from 'pinia'
import { RouterLink } from 'vue-router';
const siderStyle: CSSProperties = {
    minHeight: '100vh',
};
const sidebarMenuStyle: CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    flex: 1,
};

interface NavBar {
    label: string,
    key: string,
    icon: any,
}
function getItem(
    label: any,
    key: any,
    icon: any,
): NavBar {
    return {
        key,
        icon,
        label,
    } as NavBar;
}
const navItems: any[] = reactive([
    getItem(h(RouterLink, { to: '/home' }, () => '仪表盘主页'), 'home', () => h(DashboardOutlined)),
    getItem(h(RouterLink, { to: '/device' }, () => '设备管理'), 'device', () => h(DatabaseOutlined)),
    getItem(h(RouterLink, { to: '/tag' }, () => '标签（聚类）管理'), 'tag', () => h(TagsOutlined)),
    getItem(h(RouterLink, { to: '/api-doc', target: '_blank' }, () => 'API文档...'), 'apidoc', () => h(BookOutlined)),
]);

</script>

<script lang="ts">
export const useCollapseStore = defineStore('counter', {
    state: () => {
        return { collapsed: false }
    },
    actions: {
        toggle() {
            this.collapsed = !this.collapsed
        },
    },
})
</script>