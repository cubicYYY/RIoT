<template>
    <div :style="sloganStyle" style="background:linear-gradient(#108ee9, pink);">RIoT</div>
</template>
<script lang="ts" setup>
import type { CSSProperties } from 'vue';
const sloganStyle: CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    textAlign: 'center',
    justifyContent: 'center',
    width: '100%',
    height: '8%',
    fontSize: '2rem',
    color: 'black',
    fontStyle: 'italic',
    fontWeight: 'bold',
};
</script> 