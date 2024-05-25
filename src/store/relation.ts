import { defineStore } from 'pinia'
import type { Friend } from '@/type'
import { ref } from 'vue'
export const RelationStore = defineStore('relation', () => {
	const friends = ref<Friend[]>([])
	const friendMap = ref(new Map<number, Friend>())
	const updateMap = () => {
		for (let i of friends.value) {
			friendMap.value.set(i.id, i)
		}
	}
	return {
		friends,
		friendMap,
		updateMap,
	}
})
