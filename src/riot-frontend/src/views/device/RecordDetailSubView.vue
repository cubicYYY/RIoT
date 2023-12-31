<template>
  <a-divider orientation="left">数据</a-divider>
  <a-table
    :columns="columns"
    :data-source="data"
    v-model:pagination="pagination"
    :scroll="{ y: '40vh' }"
  />
  <a-divider orientation="left">可视化</a-divider>
  asa
</template>
<script lang="ts" setup>
const columns = [
  {
    title: 'Name',
    dataIndex: 'name',
    width: 150
  },
  {
    title: 'Age',
    dataIndex: 'age',
    width: 150
  },
  {
    title: 'Address',
    dataIndex: 'address'
  }
]

const data = [...Array(100)].map((_, i) => ({
  key: i,
  name: `Edward King ${i}`,
  age: 32,
  address: `London, Park Lane no. ${i}`
}))
let pagination = { pageSize: 50 }
</script>
