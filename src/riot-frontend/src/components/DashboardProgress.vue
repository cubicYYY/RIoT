<template>
    <a-flex align="center" justify="center" style="margin: 32px" vertical>
        <a-flex>
            <a-progress type="dashboard" stroke-linecap="square" :stroke-color="{
                '0%': HEAVY_LOAD, // wtf why is it reversed
                '100%': LIGHT_LOAD,
            }" :percent="props.percentage" :success="{ percent: 0 }" />
        </a-flex>
        <a-typography-text style="font-size: 1rem;" v-if="props.total">
            {{ props.used }} {{ props.unit }}
            / {{ props.total }} {{ props.unit }}
        </a-typography-text>
        <a-typography-title style="font-size: 1rem;">{{ props.name }}</a-typography-title>
    </a-flex>
</template>
<script lang="ts" setup>
const props = defineProps(['percentage', 'name', 'used', 'total', 'unit'])
const LIGHT_LOAD = '#003eb3';
const HEAVY_LOAD = '#780650';
</script>