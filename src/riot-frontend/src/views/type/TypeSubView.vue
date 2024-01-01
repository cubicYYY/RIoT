<template>
  <div id="tag">
    <a-alert message="TODO!! WIP" type="warning" />
    <a-flex wrap="wrap" gap="small">
      <a-card
        hoverable
        style="width: 150px; display: flex; align-items: center; justify-content: center"
        @click="showNewModal"
      >
        <!-- New device card -->
        <a-flex vertical gap="small" align="center" justify="center">
          <a-tooltip title="新增类型/数据解析器">
            <PlusCircleOutlined :style="{ fontSize: '72px', color: '#08c' }" />
          </a-tooltip>
          新增类型...
        </a-flex>
      </a-card>
      <a-card hoverable style="width: 150px" v-for="(tag, i) in allTags" :key="i">
        <template #actions>
          <a-tooltip title="编辑信息">
            <edit-outlined key="edit" @click="showEditModal"
          /></a-tooltip>
        </template>
        <a-card-meta :title="tag.name" :description="tag.description"> </a-card-meta>
        <a-typography-paragraph style="text-align: left; margin-top: 16px">
          <span>相关设备数：{{ tag.related }}</span>
        </a-typography-paragraph>
      </a-card>
    </a-flex>
    <a-modal
      v-model:open="newOpen"
      title="Basic Modal"
      :confirm-loading="confirmLoading"
      @ok="handleNewOk"
    >
      <p>Some contents...</p>
      <p>Some contents...</p>
      <p>Some contents...</p>
    </a-modal>
    <a-modal
      v-model:open="editOpen"
      title="Basic Modal"
      :confirm-loading="confirmLoading"
      @ok="handleEditOk"
    >
      <p>Some contents...</p>
      <p>Some contents...</p>
      <p>Some contents...</p>
    </a-modal>
  </div>
</template>
<script lang="ts" setup>
import { EditOutlined, PlusCircleOutlined } from '@ant-design/icons-vue'
import { ref } from 'vue'
interface Type {
  name: String
  description: String
  related: Number
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
