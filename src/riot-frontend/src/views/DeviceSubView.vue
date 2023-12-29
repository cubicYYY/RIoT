<template>
  <div id="device">
    <a-flex wrap="wrap" gap="small">
      <a-card hoverable style="width: 300px" v-for="(device, i) in allDevices" :key="i">
        <template #actions>
          <a-tooltip title="查看设备数据">
            <router-link to="/dashboard/device/6"><fund-view-outlined key="data" /></router-link></a-tooltip>
          <a-tooltip title="编辑设备信息">
            <edit-outlined key="edit" @click="showModal" /></a-tooltip>
        </template>
        <a-card-meta :title="device.name" :description="device.description"> </a-card-meta>
        <a-typography-paragraph style="text-align: left; margin-top: 16px; line-height: 2rem">
          <CardFormItem prompt="类型" :content="device.type_name"></CardFormItem>
          <CardFormItem prompt="最近更新" :content="device.last_update"></CardFormItem>
          <CardFormItem prompt="topic" :span="8">
            <template #content>
              <a-typography-paragraph copyable :content="device.topic" code style="margin: auto;">
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
    <a-modal v-model:open="open" title="编辑设备信息" :confirm-loading="confirmLoading" @ok="handleOk">
      <DevicePopup />
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import { defineAsyncComponent, ref } from 'vue'
import { EditOutlined, CopyOutlined, CopyFilled, FundViewOutlined } from '@ant-design/icons-vue'
const CardFormItem = defineAsyncComponent(() => import('@/components/CardFormItem.vue'));
const DevicePopup = defineAsyncComponent(() => import('@/views/DevicePopup.vue'));
interface Device {
  id: Number
  name: String
  description: String
  last_update: String
  type_name: String
  topic: String
}
const open = ref<boolean>(false)
const confirmLoading = ref<boolean>(false)

const showModal = () => {
  open.value = true
}
const copyTopic = () => { }
const handleOk = (e: MouseEvent) => {
  console.log(e)
  confirmLoading.value = true
  setTimeout(() => {
    open.value = false
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
