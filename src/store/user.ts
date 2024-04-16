import {defineStore} from 'pinia'
import type { UserInfo } from '@/type'
import {ref} from 'vue'
export const userStore = defineStore("user",()=>{
    const userInfo = ref<UserInfo|null>(null)
    const token = ref('')
    return {userInfo,token}
})