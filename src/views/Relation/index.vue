<template>
	<div class="relation">
		<div class="left">
			<Search v-model="friend" @search="searchFriend" />
		</div>
		<div class="right">

		</div>
	</div>
</template>

<script setup lang="ts">
import Search from '@/components/Search.vue'
import { invoke } from '@tauri-apps/api'
import { RelationStore, userStore} from '@/store'
import { ref, onMounted} from 'vue'
const useRelationStore = RelationStore()
const useUserStore = userStore()
const friend = ref('')
const flag = ref(false)
const searchFriend = () => {
	console.log(friend.value)
}
onMounted(async () => {
	await invoke('get_friend_list',{id:useUserStore.userInfo!.id})
})

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
