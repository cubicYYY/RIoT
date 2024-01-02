import { inject, ref } from 'vue'
import { defineStore } from 'pinia'
import { API_BASE_SYMBOL } from '@/type'
import axios from 'axios'
export interface User {
  id: number
  username: string
  email: string
  privilege: number
  api_key: string
  since: number
}
export const useUserStore = defineStore('user', () => {
  const api_base = inject<string>(API_BASE_SYMBOL, '/api')
  const api = axios.create({
    withCredentials: true,
    baseURL: api_base
  })
  const data = ref<null | User>(null)
  function set(userinfo: User) {
    data.value = userinfo || null
  }
  async function init(): Promise<boolean> {
    try {
      const user = (
        await api.get('/accounts/user_info', {
          headers: {
            'Content-Type': 'application/json'
          }
        })
      ).data
      set(user || {})
      return true
    } catch (error) {
      console.log(error)
      return false
    }
  }
  async function login(username: string, password: string): Promise<boolean> {
    const loginForm = {
      username,
      password
    }
    try {
      const user = (
        await api.post('/accounts/login', loginForm, {
          headers: {
            'Content-Type': 'application/json'
          }
        })
      ).data
      set(user || {})
      return true
    } catch (error) {
      console.log(error)
      return false
    }
    return false
  }
  function inited(): boolean {
    return data.value !== null
  }
  function loggedIn(): boolean {
    return data.value !== null && 'id' in data.value
  }
  return { data, set, loggedIn, login, init, inited }
})
