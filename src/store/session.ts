import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Session } from '@/type'

export const sessionStore = defineStore('session', () => {
	const sessionList = ref<Array<Session>>([])
	const current = ref(-1)
	return {
		sessionList,
		current
	}
})
