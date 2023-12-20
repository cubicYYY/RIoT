<template>
    <a-flex align="center" justify="center">
        <a-flex vertical>
            <a-flex>
                <a-progress type="dashboard" stroke-linecap="square" :stroke-color="{
                    '0%': HEAVY_LOAD, // wtf why is it reversed
                    '100%': LIGHT_LOAD,
                }" :percent="props.percentage" :success="{ percent: 0 }"/>
                <a-flex align="flex-start" vertical style="margin-left: 32px;text-align: left;" v-if="props.total">
                    <a-typography-text style="font-size: 2rem;">
                        {{ props.used }} {{ props.unit }} <br />
                        / {{ props.total }} {{ props.unit }}
                    </a-typography-text>
                </a-flex>
            </a-flex>
            <a-typography-title style="font-size: 1rem;">{{ props.name }}</a-typography-title>
        </a-flex>
    </a-flex>
</template>
<script lang="ts" setup>
const props = defineProps(['percentage', 'name', 'used', 'total', 'unit'])
const LIGHT_LOAD = '#003eb3';
const HEAVY_LOAD = '#780650';
</script>