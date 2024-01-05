<template>
  <a-flex id="user" vertical>
    <a-divider orientation="left">账号信息</a-divider>
    <CardFormItem prompt="用户名" :span="8" :content="userStore.data?.username"></CardFormItem>
    <CardFormItem prompt="UID" :span="8" :content="userStore.data?.id"></CardFormItem>
    <CardFormItem prompt="邮箱" :span="8" :content="userStore.data?.email"></CardFormItem>
    <CardFormItem prompt="权限等级" :span="8" :content="userStore.data?.privilege"></CardFormItem>
    <CardFormItem
      prompt="注册时间"
      :span="8"
      :content="new Date(userStore.data?.since || 0).toLocaleString()"
    ></CardFormItem>
    <a-divider orientation="left">API Key （设备鉴权）</a-divider>
    <a-typography-paragraph copyable :content="userStore.data?.api_key" code style="margin: auto">
      <template #copyableIcon="{ copied }">
        <CopyOutlined v-if="!copied" key="copy-icon" />
        <CopyFilled v-else key="copied-icon" />
      </template>
      <template #copyableTooltip="{ copied }">
        <span v-if="!copied" key="copy-tooltip">复制API KEY</span>
        <span v-else key="copied-tooltip">复制成功</span>
      </template>
    </a-typography-paragraph>
    <a-divider orientation="left">操作</a-divider>
    <a-row>
      <a-col :span="4"> <a-button danger @click="logout">退出登录</a-button> </a-col>
    </a-row>
  </a-flex>
</template>
<script lang="ts" setup>
import { useUserStore } from '@/stores/user'
import { defineAsyncComponent, inject } from 'vue'
const CardFormItem = defineAsyncComponent(() => import('@/components/CardFormItem.vue'))
const userStore = useUserStore()
import { CopyOutlined, CopyFilled } from '@ant-design/icons-vue'
import axios from 'axios'
import message from 'ant-design-vue/es/message'
import router from '@/router'
import { API_BASE_SYMBOL } from '@/type'
const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const api = axios.create({
  withCredentials: true,
  baseURL: api_base,
  headers: {
    'Content-Type': 'application/json'
  }
})
async function logout() {
  const response = await api.get('/accounts/logout')
  if (response.status === 200) {
    message.success('成功！')
    setTimeout(() => router.go(0), 2000) // refresh
  } else {
    message.error('?')
  }
}
</script>
