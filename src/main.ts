import { createApp } from 'vue'
import '@/styles.css'
import App from '@/App.vue'
import router from '@/routes'
import { createPinia } from 'pinia'
import 'element-plus/theme-chalk/src/message.scss'
import 'element-plus/theme-chalk/src/index.scss'
const app = createApp(App)
const store = createPinia()
app.use(router)
app.use(store)
app.mount('#app')
