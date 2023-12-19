<template>
  <a-config-provider :theme="{
    algorithm: darkModeStore.dark ? theme.darkAlgorithm : theme.defaultAlgorithm,
    token: {
      // colorPrimary: '#222222',
    },
  }">
    <RiotMain />
  </a-config-provider>
</template>
<script lang="ts" setup>
import { theme } from 'ant-design-vue';
import { useDarkModeStore } from '@/stores/dark';
const darkModeStore = useDarkModeStore()
</script>

<script lang="ts">
import { defineComponent } from "vue";
import RiotMain from "./views/DashboardView.vue";

export default defineComponent({
  components: {
    RiotMain,
  },
});
</script>