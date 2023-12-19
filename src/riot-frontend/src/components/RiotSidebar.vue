<template>
    <a-layout-sider :trigger="null" v-model:collapsed="useCollapseStore().collapsed" collapsible>
        <a-flex :style="siderStyle" vertical>
            <Slogan id="slogan"/>
            <a-menu mode="inline" theme="light" :collapsed="useCollapseStore().collapsed" :style="sidebarMenuStyle">
                <a-menu-item v-for="(item, i) in navItems" :key="i" :icon="item.icon">
                    <router-link :to="item.route"><span>{{ item.label }}</span></router-link>
                </a-menu-item>
            </a-menu>
        </a-flex>
    </a-layout-sider>
</template>

<script lang="ts" setup>
import type { CSSProperties } from 'ant-design-vue/es/_util/cssinjs/hooks/useStyleRegister';
import { h, reactive } from 'vue';
import Slogan from './RiotSlogan.vue';
import {
    DashboardOutlined,
    DatabaseOutlined,
    TagsOutlined,
    BookOutlined,
} from '@ant-design/icons-vue';
import { defineStore } from 'pinia'
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
    route: string,
}
function getItem(
    label: string,
    key: string,
    icon: any,
    route: string,
): NavBar {
    return {
        key,
        icon,
        label,
        route,
    } as NavBar;
}

const navItems: any[] = reactive([
    getItem('仪表盘主页', 'home', () => h(DashboardOutlined), '/'),
    getItem('设备管理', 'device', () => h(DatabaseOutlined), '/device'),
    getItem('标签（聚类）管理', 'tag', () => h(TagsOutlined), '/tag'),
    getItem('API文档', 'apidoc', () => h(BookOutlined), '/api-doc'),
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