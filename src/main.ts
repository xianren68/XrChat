import { createApp } from "vue"
import "@/styles.css"
import App from "@/App.vue"
import router from "@/routes"
import 'element-plus/theme-chalk/src/message.scss'
import 'element-plus/theme-chalk/src/index.scss'
const app = createApp(App)
app.use(router)
app.mount("#app")
