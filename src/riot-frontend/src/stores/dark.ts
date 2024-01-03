import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useDarkModeStore = defineStore('isDarkStore', () => {
  const dark = ref<boolean>(localStorage.getItem('dark') == 'true' || false)
  function toggle() {
    dark.value = !dark.value
    localStorage.setItem('dark', dark.value.toString())
  }
  return { dark, toggle }
})
