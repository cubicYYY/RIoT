<template>
    <a-flex id="stat-content" vertical justify="center">
        <a-divider orientation="left">服务器负载</a-divider>
        <a-row align="top">
            <a-col :span="12">
                <DashboardProgress :percentage=80 name="CPU使用率" />
            </a-col>
            <a-col :span="12">
                <DashboardProgress :percentage=60 :total=100 :used=60 name="内存使用率" unit="GB" />
            </a-col>
        </a-row>
        <v-chart class="chart" :option="loadChartOption" autoresize style="height: 300px; width: 100%;" />
        <v-chart class="chart" :option="dataChartOption" autoresize style="height: 300px; width: 100%;" />
        <v-chart class="chart" :option="deviceChartOption" autoresize style="height: 400px; width: 100%;" />
        <a-divider orientation="left">统计信息</a-divider>
        <a-flex vertical gap="middle">
            <a-row align="center" justify="start">
                <a-col :span="8">
                    <a-statistic title="服务端上线时长(uptime)" :value=114514 :formatter="timeFmter" groupSeparator="" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="系统" :value="'Ubuntu'" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="CPU Cores" :value="16" />
                </a-col>
            </a-row>
            <a-row align="center" justify="start">
                <a-col :span="8">
                    <a-statistic title="用户设备数量" :value="1000" groupSeparator="" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="在线设备(30min内活跃)" :value="13" groupSeparator="" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="用户设备数据条数" :value="114514" groupSeparator="" />
                </a-col>
            </a-row>
            <a-row align="center" justify="start">
                <a-col :span="8">
                    <a-statistic title="近1分钟负载" :value="12" groupSeparator="" suffix="%" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="近5分钟负载" :value="11" groupSeparator="" suffix="%" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="近15分钟负载" :value="16" groupSeparator="" suffix="%" />
                </a-col>
            </a-row>
        </a-flex>
    </a-flex>
</template>
<script lang="ts" setup>
import DashboardProgress from '../components/DashboardProgress.vue'
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
import type { ComposeOption } from 'echarts/core'
import type { LineSeriesOption, BarSeriesOption } from 'echarts/charts'
import type {
    TitleComponentOption,
    TooltipComponentOption,
    GridComponentOption,
    LegendComponentOption,
} from 'echarts/components'

use([
    TitleComponent,
    TooltipComponent,
    GridComponent,
    LineChart,
    CanvasRenderer,
    BarChart,
    LegendComponent,
])

type EChartsOption = ComposeOption<
    | TitleComponentOption
    | TooltipComponentOption
    | GridComponentOption
    | LineSeriesOption
    | BarSeriesOption
    | LegendComponentOption
>

import VChart from 'vue-echarts';

use([GridComponent, LineChart, CanvasRenderer, TooltipComponent, TitleComponent])

const loadChartOption: EChartsOption = {
    title: {
        left: 'center',
        text: '近30min负载'
    },
    tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'cross' }
    },
    xAxis: {
        type: 'value',
        inverse: true,
        name: '时间',
        nameLocation: 'start',
        axisLabel: {
            formatter: "{value} min"
        }
    },
    yAxis: {
        name: '负载',
        type: 'value',
        axisLabel: {
            formatter: "{value} %"
        }
    },
    series: [
        {
            data: [[15, 100], [3, 20], [2, 66], [0, 1]],
            type: 'line'
        }
    ]
};
const dataChartOption: EChartsOption = {
    title: {
        left: 'center',
        text: '近30min数据量'
    },
    tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'cross' }
    },
    xAxis: {
        type: 'value',
        inverse: true,
        name: '时间',
        nameLocation: 'start',
        axisLabel: {
            formatter: "{value} min"
        }
    },
    yAxis: {
        name: 'Record条数',
        type: 'value',
    },
    series: [
        {
            data: [[15, 100], [3, 20], [2, 66], [0, 1]],
            type: 'line'
        }
    ]
};

const deviceCountData = [
    [100, 302, 301, 334, 390, 330, 320],
    [320, 132, 101, 134, 90, 230, 210],
];
const grid = {
    left: 100,
    right: 100,
    top: 50,
    bottom: 50
};
const series = [
    '在线',
    '离线',
].map((name, sid) => {
    return {
        name,
        type: 'bar',
        stack: 'total',
        barWidth: '60%',
        label: {
            show: true,
        },
        data: deviceCountData[sid]
    };
});
const deviceChartOption = {
    title: {
        text: '设备数量'
    },
    tooltip: {
        trigger: 'axis',
        axisPointer: { type: 'cross' }
    },
    legend: {
        selectedMode: true,
    },
    grid,
    yAxis: {
        type: 'value'
    },
    xAxis: {
        type: 'category',
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
    },
    series
};

function timeFmter(_value: any) {
    return '25时Night'
}
</script>