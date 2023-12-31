<template>
  <div id="device">
    <a-flex wrap="wrap" gap="small">
      <a-card
        hoverable
        style="width: 300px; display: flex; align-items: center; justify-content: center"
        @click="showNewModal"
      >
        <!-- New device card -->
        <a-flex vertical gap="small" align="center" justify="center">
          <a-tooltip title="新增设备">
            <PlusCircleOutlined :style="{ fontSize: '72px', color: '#08c' }" />
          </a-tooltip>
          新增设备...
        </a-flex>
      </a-card>
      <a-card hoverable style="width: 300px" v-for="(device, i) in allDevices" :key="i">
        <template #actions>
          <a-tooltip title="查看设备数据">
            <router-link to="/dashboard/device/6"><fund-view-outlined key="data" /></router-link
          ></a-tooltip>
          <a-tooltip title="编辑设备信息">
            <edit-outlined key="edit" @click="showEditModal"
          /></a-tooltip>
        </template>
        <a-card-meta :title="device.name" :description="device.description"> </a-card-meta>
        <a-typography-paragraph style="text-align: left; margin-top: 16px; line-height: 2rem">
          <CardFormItem prompt="类型" :content="device.type_name"></CardFormItem>
          <CardFormItem prompt="最近更新" :content="device.last_update"></CardFormItem>
          <CardFormItem prompt="topic" :span="8">
            <template #content>
              <a-typography-paragraph copyable :content="device.topic" code style="margin: auto">
                <template #copyableIcon="{ copied }">
                  <CopyOutlined v-if="!copied" key="copy-icon" />
                  <CopyFilled v-else key="copied-icon" />
                </template>
                <template #copyableTooltip="{ copied }">
                  <span v-if="!copied" key="copy-tooltip">复制topic</span>
                  <span v-else key="copied-tooltip">复制成功</span>
                </template>
              </a-typography-paragraph>
            </template>
          </CardFormItem>
        </a-typography-paragraph>
      </a-card>
    </a-flex>
    <a-modal
      v-model:open="editOpen"
      title="编辑设备信息（留空为不变）"
      :confirm-loading="confirmLoading"
      @ok="handleEditOk"
    >
      <EditDevicePopup />
    </a-modal>
    <a-modal
      v-model:open="newOpen"
      title="新增设备"
      :confirm-loading="confirmLoading"
      @ok="handleNewOk"
    >
      <NewDevicePopup />
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import { defineAsyncComponent, inject, ref } from 'vue'
import {
  EditOutlined,
  CopyOutlined,
  CopyFilled,
  FundViewOutlined,
  PlusCircleOutlined
} from '@ant-design/icons-vue'
import { API_BASE_SYMBOL } from '@/type'
import axios from 'axios'

const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const device_api_base = api_base + '/device'
const CardFormItem = defineAsyncComponent(() => import('@/components/CardFormItem.vue'))
const EditDevicePopup = defineAsyncComponent(() => import('@/views/device/EditDevicePopup.vue'))
const NewDevicePopup = defineAsyncComponent(() => import('@/views/device/NewDevicePopup.vue'))
interface Device {
  id: Number
  name: String
  description: String
  last_update: String
  type_name: String
  topic: String
}
const editOpen = ref<boolean>(false)
const newOpen = ref<boolean>(false)
const confirmLoading = ref<boolean>(false)

const showEditModal = () => {
  editOpen.value = true
}
const showNewModal = () => {
  newOpen.value = true
}

const handleEditOk = (e: MouseEvent) => {
  console.log(e)
  confirmLoading.value = true
  setTimeout(() => {
    editOpen.value = false
    confirmLoading.value = false
  }, 2000)
}
const handleNewOk = (e: MouseEvent) => {
  console.log(e)
  confirmLoading.value = true
  setTimeout(() => {
    newOpen.value = false
    confirmLoading.value = false
  }, 2000)
}
const allDevices: Device[] = [
  {
    id: 1,
    name: 'device1',
    description: 'This is the device',
    last_update: 'Today',
    type_name: 'DHT22',
    topic: '/key/home/dht'
  },
  {
    id: 1,
    name: 'device1',
    description: 'This is the device',
    last_update: 'Today',
    type_name: 'DHT22',
    topic: '/key/home/dsssshtdsssshtdssssshtdssssht'
  },
  {
    id: 1,
    name: 'device1',
    description: 'This is the device',
    last_update: 'Today',
    type_name: 'DHT22',
    topic: '/key/home/dht'
  },
  {
    id: 1,
    name: 'device1',
    description: 'This is the device',
    last_update: 'Today',
    type_name: 'DHT22',
    topic: '/key/home/dht'
  },
  {
    id: 1,
    name: 'device1',
    description: 'This is the device',
    last_update: 'Today',
    type_name: 'DHT22',
    topic: '/key/home/dht'
  }
]
</script>
