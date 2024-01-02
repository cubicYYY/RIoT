<template>
  <div id="device">
    <a-flex wrap="wrap" gap="middle">
      <a-card
        hoverable
        style="width: 500px; display: flex; align-items: center; justify-content: center"
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
      <a-card hoverable style="width: 500px" v-for="(device, i) in allDevices" :key="i">
        <template #actions>
          <a-tooltip title="查看设备数据">
            <router-link :to="'/dashboard/device/' + device.id">
              <fund-view-outlined key="data" />
            </router-link>
          </a-tooltip>
          <a-tooltip title="编辑设备信息">
            <edit-outlined key="edit" @click="showEditModal(device.id)"
          /></a-tooltip>
        </template>
        <a-card-meta :title="device.name" :description="device.description"> </a-card-meta>
        <a-typography-paragraph style="text-align: left; margin-top: 16px; line-height: 2rem">
          <CardFormItem prompt="类型" :span="8" :content="device.dtype"></CardFormItem>
          <CardFormItem
            prompt="最近更新"
            :span="8"
            :content="timestamp2time(parseInt(device.last_update))"
          ></CardFormItem>
          <CardFormItem prompt="topic" :span="8">
            <template #content>
              <a-typography-paragraph
                copyable
                :content="apiKey + device.topic"
                code
                style="margin: auto"
              >
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
    <contextHolder ref="editPopup" />
    <a-modal
      v-model:open="newOpen"
      title="新增设备"
      :confirm-loading="confirmLoading"
      width="60vw"
      @ok="handleNewOk"
    >
      <NewDevicePopup ref="newPopup" />
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import { defineAsyncComponent, h, inject, ref } from 'vue'
import {
  EditOutlined,
  CopyOutlined,
  CopyFilled,
  FundViewOutlined,
  PlusCircleOutlined
} from '@ant-design/icons-vue'
import { API_BASE_SYMBOL } from '@/type'
import axios from 'axios'
import router from '@/router'
import message from 'ant-design-vue/es/message'
import { Modal } from 'ant-design-vue'
import { useUserStore } from '@/stores/user'
const userStore = useUserStore()
const apiKey = userStore.data?.api_key + '/'
const editPopup = ref<InstanceType<typeof EditDevicePopup>>()
const newPopup = ref<InstanceType<typeof NewDevicePopup>>()
const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const api = axios.create({
  withCredentials: true,
  baseURL: api_base
})
const CardFormItem = defineAsyncComponent(() => import('@/components/CardFormItem.vue'))
const EditDevicePopup = defineAsyncComponent(() => import('@/views/device/EditDevicePopup.vue'))
const NewDevicePopup = defineAsyncComponent(() => import('@/views/device/NewDevicePopup.vue'))
interface Device {
  id: number
  name: string
  description: string
  last_update: string
  dtype: string
  topic: string
  activated: boolean
}
const editOpen = ref<boolean>(false)
const newOpen = ref<boolean>(false)
const confirmLoading = ref<boolean>(false)
const editDid = ref<number>()
const [modal, contextHolder] = Modal.useModal()
const showEditModal = (did: number) => {
  editDid.value = did
  const init = allDevices.find((item) => item.id === did)
  modal.confirm({
    title: '编辑设备信息（留空为不变）',
    icon: h(EditOutlined),
    cancelText: '放弃修改',
    okText: 'X',
    width: '60vw',
    content: h(EditDevicePopup, { did, init }),
    class: 'edit-device',
    okButtonProps: { disabled: true }
  })
  editOpen.value = true
}
const showNewModal = () => {
  newOpen.value = true
}

const handleNewOk = async (e: MouseEvent) => {
  confirmLoading.value = true
  if (newPopup.value !== null) {
    const response = await newPopup.value!.submit()
    if (response.status === 200) {
      message.success('添加成功！')
      setTimeout(() => router.go(0), 2000) // refresh
    } else {
      message.error('添加失败！ 请检查是否有重复的topic或名称: ' + response.data.message)
    }
  }
  confirmLoading.value = false
  newOpen.value = false
}
function timestamp2time(timestamp: number) {
  const iso = new Date(timestamp)
  return (
    iso.getFullYear() +
    '-' +
    (iso.getMonth() + 1) +
    '-' +
    iso.getDate() +
    ' ' +
    iso.getHours() +
    ':' +
    iso.getMinutes() +
    ':' +
    iso.getSeconds()
  )
}
const allDevicesRaw: Device[] = (await api.get('/devices')).data
const allDevices: Device[] = allDevicesRaw.filter((item) => item.activated)
</script>
