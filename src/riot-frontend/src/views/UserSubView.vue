<template>
  <a-flex id="user" vertical>
    <a-divider orientation="left">账号信息</a-divider>
    <CardFormItem
            prompt="用户名"
            :span="8"
            :content="userStore.data?.username"
          ></CardFormItem>
    <a-divider orientation="left">API Key</a-divider>
  </a-flex>
</template>
<script lang="ts" setup>
import { useUserStore } from '@/stores/user'
import { defineAsyncComponent } from 'vue';
const CardFormItem = defineAsyncComponent(() => import('@/components/CardFormItem.vue'))
const userStore = useUserStore()

</script>