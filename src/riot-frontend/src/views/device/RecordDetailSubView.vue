<template>
  <a-flex vertical>
    <a-divider orientation="left">数据</a-divider>
    <a-table :columns="columns" :data-source="recordData" :scroll="{ x: 2000, y: 800 }" />
    <a-divider orientation="left">可视化折线图</a-divider>
    <template v-for="(line, i) in chartOptions" :key="i">
      <v-chart class="chart" :option="line" autoresize style="height: 300px; width: 100%" />
    </template>
    <a-divider orientation="left">可视化地图（点击查看数据）</a-divider>
    <div id="amap" style="height: 600px; width: 100%"></div>
  </a-flex>
</template>
<script lang="ts" setup>
import { API_BASE_SYMBOL } from '@/type'
import axios from 'axios'
import { inject, onMounted, reactive } from 'vue'
import VChart from 'vue-echarts'
import { use } from 'echarts/core'
import { LineChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers'
import { parse, parsers, type Parser, type RiotMap } from '@/parser'
import AMapLoader from '@amap/amap-jsapi-loader'
import message from 'ant-design-vue/es/message'
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
const recordData: any[] = datas.map(({ id, payload, timestamp }) => {
  const human_time = new Date(timestamp).toLocaleString()
  return { id, ...parse(payload, dtype), time: human_time }
})

let columns = [
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
if (recordData.length > 0) {
  // Update with actual data format
  columns = Object.keys(recordData[0]).map((key: any) => ({
    title: key,
    dataIndex: key,
  }))
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
      type: 'value',
      min: 'dataMin',
      max: 'dataMax'
    },
    yAxis: {
      name: 'y',
      type: 'value',
      min: 'dataMin',
      max: 'dataMax'
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
// console.log(chartOptions)
const state = reactive<any>({
  map: null, // 地图实例
  marker: null, // 地图icon
  address: '杭州市'
})

declare global {
  interface Window {
    _AMapSecurityConfig: any
  }
}

const initMap = () => {
  window._AMapSecurityConfig = {
    // DEBUG ONLY
    securityJsCode: '90c3cb3dcd399363b157b42345c5ddf6' // 高德Secure Key
  }
  function openInfo(AMap: any, position: any, data: any) {
    let infoWindow = new AMap.InfoWindow({
      content: data,
      position
    })
    infoWindow.open(state.map)
  }
  AMapLoader.load({
    key: '94f0a59afe2573ae1634b5d58dab49da', // 高德Web Key
    version: '2.0',
    plugins: [
      'AMap.Geocoder', // 逆向地理解码插件
      'AMap.Marker' // 点标记
    ]
  })
    .then((AMap) => {
      state.map = new AMap.Map('amap', {
        viewMode: '3D',
        zoom: 12
      })
      let pointLayer = new AMap.LabelsLayer({
        zooms: [3, 20],
        zIndex: 111,
        animation: false,
        collision: false
      })
      state.map.add(pointLayer)
      const markers = parser.maps?.map((gdmap: RiotMap) =>
        recordData
          // .sort((a, b) => a[gdmap.order] || a.timestamp - b[gdmap.order] || a.timestamp)
          .map((obj: any) => [obj[gdmap.longitude], obj[gdmap.latitude]])
      )
      let icon = {
        type: 'image',
        image: 'https://webapi.amap.com/theme/v1.3/markers/n/mark_b.png',
        size: [12, 18],
        anchor: 'bottom-center',
        angel: 0,
        retina: true
      }
      let normalMarker = new AMap.Marker({
        offset: new AMap.Pixel(-75, -40)
      })
      let mapMarkers: any[] = []
      if (markers && markers.length > 0) {
        // Currently support 1 map
        for (let i = 0; i < markers[0].length; i++) {
          var curPosition = markers[0][i]
          var curData = {
            position: curPosition,
            icon,
            extData: { id: i }
          }
          let labelMarker = new AMap.LabelMarker(curData)
          labelMarker.on('click', function (e: any) {
            var position = e.data.data && e.data.data.position
            let index = e.target.getExtData().id
            console.log(recordData[index])
            openInfo(AMap, position, JSON.stringify(recordData[index], null, 2))
          })

          labelMarker.on('mouseout', function () {
            state.map.remove(normalMarker)
          })

          mapMarkers.push(labelMarker)
        }
        pointLayer.add(mapMarkers)
        new AMap.Polyline({
          map: state.map,
          path: markers[0],
          showDir: true,
          strokeColor: '#28F',
          strokeWeight: 2
        })
      }
      state.map.setFitView()
    })
    .catch((e) => {
      console.log(e)
      message.error(e)
    })
}
onMounted(initMap)
</script>
