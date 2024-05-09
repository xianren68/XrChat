import {defineStore} from 'pinia'
import type{ Friend } from '@/type'
import { ref } from 'vue'
export const RelationStore = defineStore('relation',()=> {
    const friends = ref<Friend[]>([])
    return {friends}
})