import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useDarkModeStore = defineStore('isDarkStore', () => {
  const dark = ref(false)
  function toggle() {
    dark.value = !dark.value
  }
  return { dark, toggle }
})
