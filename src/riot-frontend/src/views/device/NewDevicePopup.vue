<template>
  <a-form
    :model="formState"
    name="basic"
    :label-col="{ span: 6 }"
    :wrapper-col="{ span: 12 }"
    autocomplete="off"
  >
    <a-form-item
      label="设备名称"
      name="name"
      :rules="[{ required: true, message: '请输入设备名' }]"
    >
      <a-input v-model:value="formState.name" required />
    </a-form-item>

    <a-form-item label="设备描述" name="description">
      <a-textarea v-model:value="formState.desc" />
    </a-form-item>

    <a-form-item
      label="设备类型"
      name="dtype"
      :span="8"
      :rules="[{ required: true, message: '请指定设备类型' }]"
    >
      <a-input v-model:value="formState.dtype" />
    </a-form-item>

    <a-form-item label="纬度（可选）" name="latitude">
      <a-input-number v-model:value="formState.latitude" />
    </a-form-item>

    <a-form-item label="经度（可选）" name="longitude">
      <a-input-number v-model:value="formState.longitude" />
    </a-form-item>

    <a-form-item
      label="topic"
      name="topic"
      :rules="[{ required: true, message: '请输入topic后缀' }]"
      :autoLink="false"
    >
      <a-input-group compact>
        <a-form-item-rest>
          <a-tooltip title="API Key为固定前缀">
            <a-input :value="apiKey" disabled style="width: 30%" />
          </a-tooltip>
        </a-form-item-rest>
        <a-input v-model:value="formState.topic" style="width: 70%" />
      </a-input-group>
    </a-form-item>
  </a-form>
</template>
<script lang="ts" setup>
import { API_BASE_SYMBOL } from '@/type'
import axios, { type AxiosResponse } from 'axios'
import { inject, reactive } from 'vue'
const apiKey = 114514
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
async function newDevice(form: FormState): Promise<AxiosResponse<any, any>> {
  try {
    const response = await api.post('/devices', form, {
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
const formState = reactive<FormState>({
  name: '',
  desc: '',
  dtype: 1,
  latitude: null,
  longitude: null,
  topic: '/test'
})
const submit = async (): Promise<AxiosResponse<any, any>> => {
  console.log(formState)
  const response = await newDevice(formState)
  return response
}

defineExpose({
  submit
})
</script>
