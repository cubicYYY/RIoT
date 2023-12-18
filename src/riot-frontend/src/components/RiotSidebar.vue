<template>
    <a-layout-sider :trigger="null" v-model:collapsed="useCollapseStore().collapsed" collapsible>
        <Slogan id="slogan"/>
        <a-menu mode="inline" :collapsed="useCollapseStore().collapsed">
            <a-menu :items="navItems"></a-menu>
        </a-menu>
    </a-layout-sider>
</template>

<script lang="ts" setup>
import { h, reactive } from 'vue';
import Slogan from './RiotSlogan.vue';
import {
    HomeOutlined,
    DatabaseOutlined,
    TagsOutlined,
    InfoCircleOutlined,
    BookOutlined,
} from '@ant-design/icons-vue';
import type { ItemType } from 'ant-design-vue';
import { defineStore } from 'pinia'

function getItem(
    label: string,
    key: string,
    icon?: any,
    children?: ItemType[],
    type?: 'group',
): ItemType {
    return {
        key,
        icon,
        children,
        label,
        type,
    } as ItemType;
}

const navItems: ItemType[] = reactive([
    getItem('首页', 'home', h(HomeOutlined)),
    getItem('设备管理', 'device', h(DatabaseOutlined)),
    getItem('标签（聚类）管理', 'tag', h(TagsOutlined)),
    getItem('站点状态', 'site', h(InfoCircleOutlined)),
    getItem('API文档', 'apidoc', h(BookOutlined)),
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