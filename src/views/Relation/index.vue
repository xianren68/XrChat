<template>
	<div class="relation">
		<div class="left">
			<Search v-model="searchValue" @search="handleSearch" @cancel="handleCancel" :search="flag" />
			<Switch :values="values" @selected="selected" :select="select" />
		</div>
		<div class="right"></div>
	</div>
</template>

<script setup lang="ts">
import Search from '@/components/Search.vue'
import Switch from '@/components/Switch.vue'
import { invoke } from '@tauri-apps/api'
import { RelationStore, userStore } from '@/store'
import { ref, onMounted } from 'vue'
const useRelationStore = RelationStore()
const useUserStore = userStore()
const searchValue = ref('')

const values = ['好友', '群组']
// isSearch mode
const flag = ref(true)
const select = ref(values[0])
const handleSearch = () => {
	// TODO: search
	flag.value = false
	console.log(searchValue.value)
}
const handleCancel = () => {
	// TODO: cancel search
	flag.value = true
	searchValue.value = ''
}
onMounted(async () => {
	await invoke('get_friend_list', { id: useUserStore.userInfo!.id })
})

const selected = (value: string) => {
	select.value = value
}
</script>

<style scoped>
.relation {
	box-sizing: border-box;
	width: calc(100% - 60px);
	height: 100%;
	display: flex;
	.left {
		height: 100%;
		width: 300px;
		padding: 10px;
		box-sizing: border-box;
		.notice-active {
			transform: scale(1.03);
			box-shadow: 2px 2px 2px rgba(0, 0, 0, 0.2);
		}
	}
}
</style>
