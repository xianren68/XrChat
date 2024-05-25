import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Session } from '@/type'

export const sessionStore = defineStore('session', () => {
	const sessionList = ref<Array<Session>>([])
	const sessonMap = ref(new Map<number, Session>())
	const current = ref(-1)
	const updateMap = () => {
		for (let i of sessionList.value) {
			sessonMap.value.set(i.id, i)
		}
	}
	return {
		sessionList,
		current,
		sessonMap,
		updateMap,
	}
})
