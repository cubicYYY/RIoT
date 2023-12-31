import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import Antd from 'ant-design-vue'
import 'ant-design-vue/dist/reset.css'
import { API_BASE_SYMBOL } from './type'
import App from './App.vue'
const app = createApp(App)
app.use(createPinia())
app.use(router)
app.use(Antd)
app.provide(API_BASE_SYMBOL, API_BASE)
app.mount('#app')
import router from './router'
import { API_BASE } from './config'
