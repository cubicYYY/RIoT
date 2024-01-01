<template>
  <a-form
    :model="formState"
    name="basic"
    :label-col="{ span: 6 }"
    :wrapper-col="{ span: 12 }"
    autocomplete="off"
  >
    <a-form-item label="设备名称" name="name">
      <a-input v-model:value="formState.name" />
    </a-form-item>

    <a-form-item label="设备描述" name="description">
      <a-textarea v-model:value="formState.desc" />
    </a-form-item>

    <a-form-item label="设备类型" name="dtype" :span="8">
      <a-input v-model:value="formState.dtype" />
    </a-form-item>

    <a-form-item label="纬度（可选）" name="latitude">
      <a-input-number v-model:value="formState.latitude" />
    </a-form-item>

    <a-form-item label="经度（可选）" name="longitude">
      <a-input-number v-model:value="formState.longitude" />
    </a-form-item>

    <a-form-item label="topic" name="topic">
      <a-input-group compact>
        <a-form-item-rest
          ><a-tooltip title="API Key为固定前缀">
            <a-input :value="apiKey" disabled style="width: 30%" />
          </a-tooltip>
        </a-form-item-rest>
        <a-input v-model:value="formState.topic" style="width: 70%" />
      </a-input-group>
    </a-form-item>

    <a-form-item :wrapper-col="{ span: 4, offset: 10 }">
      <a-button type="primary" @click.prevent="onSubmit">提交修改</a-button>
    </a-form-item>
  </a-form>
</template>
<script lang="ts" setup>
import router from '@/router'
import { API_BASE_SYMBOL } from '@/type'
import { message } from 'ant-design-vue'
import type { AxiosResponse } from 'axios'
import axios from 'axios'
import { inject, reactive } from 'vue'
const props = defineProps(['did', 'init'])
console.log(props.did)
const apiKey = '114514'

const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const api = axios.create({
  withCredentials: true,
  baseURL: api_base
})
interface FormState {
  name: string
  desc: string
  dtype: Number
  latitude: Number | null
  longitude: Number | null
  topic: string
}
async function editDevice(form: FormState): Promise<AxiosResponse<any, any>> {
  try {
    const response = await api.put('/devices/' + props.did, form, {
      headers: {
        'Content-Type': 'application/json'
      }
    })
    return response
  } catch (error: any) {
    console.log(error)
    return error.response
  }
}
const formState = reactive<FormState>(props.init)
const onSubmit = async (): Promise<void> => {
  console.log(formState)
  const response = await editDevice(formState)
  if (response.status === 200) message.success('修改成功！')
  else message.error('修改失败：请检查是否有重复的名称/topic')
  setTimeout(() => router.go(0), 2000) // refresh
}

defineExpose({
  submit: onSubmit
})
</script>
