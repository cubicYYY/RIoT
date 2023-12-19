<template>
  <RiotMain />
</template>
  
<script lang="ts">
import { defineComponent } from "vue";
import RiotMain from "./views/DashboardView.vue";

export default defineComponent({
  components: {
    RiotMain,
  },
});
</script>