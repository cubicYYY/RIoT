<template>
  <div id="tag">
    <a-flex wrap="wrap" gap="small">
      <a-card hoverable style="width: 150px" v-for="(tag, i) in allTags" :key="i">
        <template #actions>
          <a-tooltip title="编辑信息"> <edit-outlined key="edit" @click="showModal" /></a-tooltip>
        </template>
        <a-card-meta :title="tag.name" :description="tag.description"> </a-card-meta>
        <a-typography-paragraph style="text-align: left; margin-top: 16px">
          <span>相关设备数：{{ tag.related }}</span>
        </a-typography-paragraph>
      </a-card>
    </a-flex>
    <a-modal
      v-model:open="open"
      title="Basic Modal"
      :confirm-loading="confirmLoading"
      @ok="handleOk"
    >
      <p>Some contents...</p>
      <p>Some contents...</p>
      <p>Some contents...</p>
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import { EditOutlined, DatabaseOutlined } from '@ant-design/icons-vue'
import { ref } from 'vue'
interface Type {
  name: String
  description: String
  related: Number
}
const open = ref<boolean>(false)
const confirmLoading = ref<boolean>(false)
const showModal = () => {
  open.value = true
}

const handleOk = (e: MouseEvent) => {
  console.log(e)
  confirmLoading.value = true
  setTimeout(() => {
    open.value = false
    confirmLoading.value = false
  }, 2000)
}
const allTags: Type[] = [
  {
    name: 'Type Name',
    description: 'This is Makise Kurisu',
    related: 8
  },
  {
    name: 'Type Name',
    description: 'This is Makise Kurisu',
    related: 8
  },
  {
    name: 'Type Name',
    description: 'This is Makise Kurisu',
    related: 8
  },
  {
    name: 'Type Name',
    description: 'This is Makise Kurisu',
    related: 8
  },
  {
    name: 'Type Name',
    description: 'This is Makise Kurisu',
    related: 8
  }
]
</script>
