<template>
    <div id="stat-content">
        <a-divider orientation="left">服务器负载</a-divider>
        <a-row align="middle">
            <a-col :span="12">
                <DashboardProgress :percentage=80 name="CPU使用率" />
            </a-col>
            <a-col :span="12">
                <DashboardProgress :percentage=60 :total=100 :used=60 name="内存使用率" unit="GB" />
            </a-col>
        </a-row>
        <a-divider orientation="left">统计信息</a-divider>
        <a-flex vertical gap="middle">
            <a-row align="middle" justify="start">
                <a-col :span="8">
                    <a-statistic title="服务端上线时长(uptime)" :value=114514 :formatter="timeFmter"
                        groupSeparator="" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="系统" :value="'Ubuntu'" />
                </a-col>
                <a-col :span="8">
                    <a-statistic title="CPU Cores" :value="16" />
                </a-col>
            </a-row>
            <a-row align="middle" justify="start">
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
            <a-row align="middle" justify="start">
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
    </div>
</template>
<script lang="ts" setup>
import DashboardProgress from '../components/DashboardProgress.vue'
function timeFmter(_value: any) {
    return '25时Night'
}
</script>