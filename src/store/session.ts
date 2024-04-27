import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Session } from '@/type'

export const sessionStore = defineStore('session', () => {
	const sessionList = ref<Array<Session>>([])
	return {
		sessionList,
	}
})