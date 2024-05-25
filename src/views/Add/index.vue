<template>
	<div class="add">
		<div class="left">
			<Search v-model="key" @search="handleSearch"></Search>
		</div>
		<div class="right"></div>
	</div>
</template>

<script setup lang="ts">
import Search from '@/components/Search.vue'
import { ref } from 'vue'
import { SearchReq } from '@/api'
import { relation } from '@/pb/relation'
import { userStore, RelationStore } from '@/store'
const useUserStore = userStore()
const useRelationStore = RelationStore()
const key = ref()
const groupList = ref([])
const userList = ref([])
const handleSearch = async () => {
	const arrayBuffer = await SearchReq(
		new relation.SearchReq({
			id: useUserStore.userInfo!.id,
			key: key.value,
		})
	)
	const resp = relation.SearchRes.deserialize(arrayBuffer as Uint8Array)
}
const handleGroup = (groups: relation.Res[]) => {
	for (let i of groups) {
	}
}
const handleUser = (users: relation.Res[]) => {
	for (let i of users) {
		if (i.id === useUserStore.userInfo!.id) {
			continue
		}
		if (useRelationStore.friendMap.has(i.id)) {
			continue
		}
	}
}
</script>

<style lang="scss" scoped>
.add {
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
	.right {
		height: 100%;
		flex: 1;
	}
}
</style>
