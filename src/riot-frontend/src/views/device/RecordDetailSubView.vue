<template>
  <div>
    <a-divider orientation="left">数据</a-divider>
    <a-table
      :columns="columns"
      :data-source="recordData"
      v-model:pagination="pagination"
      :scroll="{ y: '40vh' }"
    />
    <a-divider orientation="left">可视化</a-divider>
    <template v-for="(line, i) in chartOptions" :key="i">
      <v-chart class="chart" :option="line" autoresize style="height: 300px; width: 100%" />
    </template>
    <!-- <v-chart class="chart" :option="mapChartOption" autoresize style="height: 300px; width: 100%" /> -->
  </div>
</template>
<script lang="ts" setup>
import { API_BASE_SYMBOL } from '@/type'
import axios from 'axios'
import { computed, inject } from 'vue'
import VChart from 'vue-echarts'
import { use } from 'echarts/core'
import { LineChart, BarChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'
import { parse, parsers, type Parser } from '@/parser'
use([GridComponent, LineChart, CanvasRenderer, TooltipComponent, TitleComponent, LegendComponent])

interface Record {
  id: number
  did: number
  payload: number[]
  timestamp: number
}
const api_base = inject<string>(API_BASE_SYMBOL, '/api')
const api = axios.create({
  withCredentials: true,
  baseURL: api_base,
  headers: {
    'Content-Type': 'application/json'
  }
})
const props = defineProps(['id'])
const device = (await api.get('/devices/' + props.id)).data
const dtype = device.dtype
const datas: Record[] = (await api.get('/devices/' + props.id + '/records')).data
const recordData = datas.map(({ id, payload, timestamp }) => {
  const human_time = new Date(timestamp).toLocaleString()
  return { id, ...parse(payload, dtype), time: human_time }
})
console.log(recordData)
const columns = [
  {
    title: 'ID',
    dataIndex: 'id'
  },
  {
    title: 'Raw Payload',
    dataIndex: 'payload'
  },
  {
    title: 'Time',
    dataIndex: 'time'
  }
]
let pagination = { pageSize: 50 }
const grid = {
  left: 100,
  right: 100,
  top: 50,
  bottom: 50
}
const lineChartOption = {
  title: {
    left: 'center',
    text: '近30min负载'
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'cross' }
  },
  xAxis: {
    name: 'x',
    type: 'value'
  },
  yAxis: {
    name: 'y',
    type: 'value'
  },
  series: [
    // {
    //   data: cpuUsage30min.value,
    //   type: 'line',
    // }
  ]
}
const parser: Parser = parsers[dtype]
const chartOptions = parser.lines?.map((line) => {
  const chartOption = {
    title: {
      left: 'center',
      text: line.title
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' }
    },
    xAxis: {
      name: 'x',
      type: 'value'
    },
    yAxis: {
      name: 'y',
      type: 'value'
    },
    series: [
      {
        data: recordData.map((obj: any) => [obj[line.x], obj[line.y]]),
        type: 'line'
      }
    ]
  }
  return chartOption
})
console.log(chartOptions)
const mapChartOption = computed(() => ({
  title: {
    text: '设备数量'
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'shadow' }
  },
  legend: {
    selectedMode: true
  },
  grid,
  yAxis: {
    type: 'value'
  },
  xAxis: {
    min: 0,
    max: 30,
    type: 'value',
    inverse: true,
    name: '时间',
    nameLocation: 'start',
    axisLabel: {
      formatter: '{value} min'
    }
  },
  series: recordData
}))
</script>
