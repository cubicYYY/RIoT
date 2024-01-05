<template>
  <a-flex id="stat-content" vertical justify="center">
    <a-divider orientation="left">服务器负载</a-divider>
    <a-row align="top">
      <a-col :span="12">
        <DashboardProgress :percentage="status['cpu_usage'].toFixed(2)" name="CPU使用率" />
      </a-col>
      <a-col :span="12">
        <DashboardProgress
          :percentage="mem_usage.toFixed(2)"
          :total="mem_total.toFixed(2)"
          :used="mem_used.toFixed(2)"
          name="内存使用率"
          unit="GB"
        />
      </a-col>
    </a-row>
    <v-chart
      class="chart"
      :option="loadChartOption"
      autoresize
      style="height: 300px; width: 100%"
    />
    <v-chart
      class="chart"
      :option="dataChartOption"
      autoresize
      style="height: 300px; width: 100%"
    />
    <v-chart
      class="chart"
      :option="deviceChartOption"
      autoresize
      style="height: 400px; width: 100%"
    />
    <a-divider orientation="left">统计信息</a-divider>
    <a-flex vertical gap="middle">
      <a-row align="center" justify="start">
        <a-col :span="8">
          <a-statistic
            title="服务器uptime"
            :value="uptime"
            :formatter="timeFmter"
            groupSeparator=""
          />
        </a-col>
        <a-col :span="8">
          <a-statistic title="系统" :value="status['sys_name']" />
        </a-col>
        <a-col :span="8">
          <a-statistic title="CPU Cores" :value="status['cpu_core_count']" />
        </a-col>
      </a-row>
      <a-row align="center" justify="start">
        <a-col :span="8">
          <a-statistic
            title="SWAP区可用大小"
            :value="swap_free.toFixed(2)"
            groupSeparator=""
            suffix="MB"
          />
        </a-col>
        <a-col :span="8">
          <a-statistic
            title="SWAP区已使用"
            :value="swap_used.toFixed(2)"
            groupSeparator=""
            suffix="MB"
          />
        </a-col>
        <a-col :span="8">
          <a-statistic
            title="SWAP区总大小"
            :value="swap_total.toFixed(2)"
            groupSeparator=""
            suffix="MB"
          />
        </a-col>
      </a-row>
      <a-row align="center" justify="start">
        <a-col :span="8">
          <a-statistic
            title="近1分平均负载"
            :value="status['load_avg_1_5_15'][0]"
            groupSeparator=""
            suffix="%"
          />
        </a-col>
        <a-col :span="8">
          <a-statistic
            title="近5分钟平均负载"
            :value="status['load_avg_1_5_15'][1]"
            groupSeparator=""
            suffix="%"
          />
        </a-col>
        <a-col :span="8">
          <a-statistic
            title="近15分钟平均负载"
            :value="status['load_avg_1_5_15'][2]"
            groupSeparator=""
            suffix="%"
          />
        </a-col>
      </a-row>
    </a-flex>
  </a-flex>
</template>
<script lang="ts" setup>
import DashboardProgress from '@/components/DashboardProgress.vue'
/* Echarts */
import { use } from 'echarts/core'
import { LineChart, BarChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'
import axios from 'axios'
const UPDATE_INTERVAL = 12.0
function index2minute(length: number, index: number): number {
  return (length - index - 1) * (UPDATE_INTERVAL / 60.0)
}
const api_base = inject<string>(API_BASE_SYMBOL, '/api')
let status_ref = ref((await axios.get(api_base + '/healthchecker')).data)
const status = status_ref.value
let mem_total = ref(status['mem_total'] / 1024 / 1024 / 1024) // Bytes to GB
let mem_used = ref((status['mem_total'] - status['mem_available']) / 1024 / 1024 / 1024) // Bytes to GB
let mem_usage = ref((100 * (status['mem_total'] - status['mem_available'])) / status['mem_total'])
let swap_free = ref(status['swap_free'] / 1024 / 1024) // Bytes to MB
let swap_total = ref(status['swap_total'] / 1024 / 1024) // Bytes to MB
let swap_used = ref((status['swap_total'] - status['swap_free']) / 1024 / 1024) // Bytes to MB
let cpuUsage30min = ref(
  (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['cpu_usage']
  ])
)
let recordCount30min = ref(
  (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['record_count']
  ])
)
let offlineCount30min = ref(
  (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['device_count'] - item['device_online']
  ])
)
let onlineCount30min = ref(
  (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['device_online']
  ])
)
let uptime = ref(status['uptime'])
use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LineChart,
  CanvasRenderer,
  BarChart,
  LegendComponent
])

import VChart from 'vue-echarts'
import { API_BASE_SYMBOL } from '@/type'
import { inject, ref, watch, type Ref, computed } from 'vue'

use([GridComponent, LineChart, CanvasRenderer, TooltipComponent, TitleComponent])

const loadChartOption = computed(() => ({
  title: {
    left: 'center',
    text: '近30min负载'
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'cross' }
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
  yAxis: {
    name: '负载',
    type: 'value',
    min: 0,
    max: 100,
    axisLabel: {
      formatter: '{value} %'
    }
  },
  series: [
    {
      data: cpuUsage30min.value,
      type: 'line',
      areaStyle: {}
    }
  ]
}))
const dataChartOption = computed(() => ({
  title: {
    left: 'center',
    text: '近30min数据量'
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'cross' }
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
  yAxis: {
    name: 'Record条数',
    type: 'value'
  },
  series: [
    {
      data: recordCount30min.value,
      type: 'bar'
    }
  ]
}))

const deviceCountData: Ref<any[]> = ref([offlineCount30min.value, onlineCount30min.value])
const grid = {
  left: 100,
  right: 100,
  top: 50,
  bottom: 50
}
const series = computed(() =>
  ['离线', '在线'].map((sname, sid) => {
    return {
      name: sname,
      type: 'bar',
      stack: 'total',
      barWidth: '60%',
      data: deviceCountData.value[sid]
    }
  })
)
const deviceChartOption = computed(() => ({
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
  series: series.value
}))

watch(status_ref, async (status) => {
  mem_total.value = status['mem_total'] / 1024 / 1024 / 1024 // Bytes to GB
  mem_used.value = (status['mem_total'] - status['mem_available']) / 1024 / 1024 / 1024 // Bytes to GB
  mem_usage.value = (100 * (status['mem_total'] - status['mem_available'])) / status['mem_total']
  swap_free.value = status['swap_free'] / 1024 / 1024 // Bytes to MB
  swap_total.value = status['swap_total'] / 1024 / 1024 // Bytes to MB
  swap_used.value = (status['swap_total'] - status['swap_free']) / 1024 / 1024 // Bytes to MB
  cpuUsage30min.value = (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['cpu_usage']
  ])
  recordCount30min.value = (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['record_count']
  ])
  offlineCount30min.value = (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['device_count']
  ])
  onlineCount30min.value = (status['last_30min'] as any[]).map((item, index) => [
    index2minute(status['last_30min'].length, index),
    item['device_online']
  ])
  uptime.value = status['uptime']
})
function increaseUptime() {
  uptime.value++
}
async function updateServerStatus() {
  status_ref.value = (await axios.get(api_base + '/healthchecker')).data
}
setInterval(updateServerStatus, 1000)
setInterval(increaseUptime, UPDATE_INTERVAL * 1000)
function timeFmter(seconds: { value: number }) {
  return new Date(seconds.value * 1000).toISOString().substring(11, 19)
}
</script>
