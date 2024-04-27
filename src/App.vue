<template>
	<div class="app">
		<Aside v-if="route.name !== 'register' && route.name !== 'login'"></Aside>
		<router-view></router-view>
	</div>
</template>

<script setup lang="ts">
import Aside from '@/components/Aside.vue'
import { useRoute } from 'vue-router'
const route = useRoute()
</script>

<style scoped>
.app {
	box-sizing: border-box;
	display: flex;
	padding: 20px;
	height: 100%;
	width: 100%;
}
</style>
