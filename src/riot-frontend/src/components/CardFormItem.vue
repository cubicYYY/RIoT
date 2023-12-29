<template>
    <a-row :gutter="gutter" align="middle">
        <a-col :span="span" style="text-align: right;">{{ props.prompt }}:</a-col>
        <a-col :span="24 - span">
            <slot v-if="!props.content" name="content"></slot>
            <div v-else>
                {{ props.content }}
            </div>
        </a-col>
    </a-row>
</template>
<script lang="ts" setup>
const props = defineProps(['prompt', 'content', 'span', 'gutter'])
let gutter = props.gutter || 16;
let span = props.span || 12;
</script>