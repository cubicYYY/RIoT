<template>
  <div>
    <a-result
      status="success"
      title="激活成功！"
      sub-title="请返回登录页面登录"
      v-if="!pending && ok"
    >
      <template #extra>
        <a-button key="login" type="primary"
          ><router-link to="/login">去登录</router-link></a-button
        >
      </template>
    </a-result>
    <a-result status="error" title="激活失败" sub-title="..." v-if="!pending && !ok">
      <template #extra>
        <a-button key="error" type="primary"
          ><router-link to="/login">去登录</router-link></a-button
        >
      </template>
    </a-result>
    <a-spin size="large" v-if="pending" />
  </div>
</template>
<script lang="ts" setup>
import { API_BASE_SYMBOL } from '@/type'
import axios from 'axios'
import { inject, ref } from 'vue'
import { useRoute } from 'vue-router'
const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const api = axios.create({
  withCredentials: true,
  baseURL: api_base
})
const route = useRoute()
console.log(route.query)
const pending = ref(true)
const ok = ref(false)
const verify = async (code: any): Promise<boolean> => {
  try {
    await api.get('/accounts/verify', {
      headers: {
        'Content-Type': 'application/json'
      },
      params: {
        code
      }
    })
    ok.value = true
    pending.value = false
    return true
  } catch (error) {
    ok.value = false
    pending.value = false
    console.log(error)
    return false
  }
}
await verify(route.query.code)
</script>
