<template>
    <a-layout-sider 
        :trigger="null" 
        v-model:collapsed="useCollapseStore().collapsed" 
        :style="siderStyle"
        breakpoint="lg" 
        collapsible>
        <div class="logo" />
        <a-flex vertical style="height: 100%;">
            <Slogan id="slogan" />
            <a-menu id="side-bar-menu-main" mode="inline" theme="light" :style="sidebarMenuStyle" :items="navItems">
            </a-menu>
            <a-menu :selectable="false" id="side-bar-menu-main" mode="inline" theme="light" :style="sidebarBottomMenuStyle"
                :items="navBottomItems">
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
    SettingOutlined,
    ApartmentOutlined,
} from '@ant-design/icons-vue';
import { defineStore } from 'pinia'
import { RouterLink } from 'vue-router';
const siderStyle: CSSProperties = {
    height: '100vh',
    position: 'sticky',
    top: 0,
    flex: 1,
};
const sidebarMenuStyle: CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    flex: 1,
};
const sidebarBottomMenuStyle: CSSProperties = {
    marginTop: 'auto',
    display: 'flex',
    flexDirection: 'column',
};
interface NavBar {
    label: string,
    key: string,
    icon: any,
    children: any[],
}
function getItem(
    label: any,
    key: any,
    icon: any,
    children?: any[],
): NavBar {
    return {
        label,
        key,
        icon,
        children,
    } as NavBar;
}
const navItems: any[] = reactive([
    getItem(h(RouterLink, { to: '/dashboard' }, () => '仪表盘主页'), 'home', () => h(DashboardOutlined)),
    getItem(h(RouterLink, { to: '/dashboard/device' }, () => '设备管理'), 'device', () => h(DatabaseOutlined)),
    getItem(h(RouterLink, { to: '/dashboard/type' }, () => '数据格式配置'), 'type', () => h(ApartmentOutlined)),
    getItem(h(RouterLink, { to: '/dashboard/tag' }, () => '标签（聚类）管理'), 'tag', () => h(TagsOutlined)),
    getItem(h(RouterLink, { to: '/dashboard/user' }, () => '账号设置'), 'user', () => h(SettingOutlined)),
]);

const navBottomItems: any[] = reactive([
    getItem(h('a', { href: '/api-doc', target: '_blank' }, 'API文档...'), 'apidoc', () => h(BookOutlined)),
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