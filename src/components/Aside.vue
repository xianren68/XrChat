<template>
	<div class="aside">
		<div class="avatar" @click="pushSelf">
			<img src="" alt="" />
		</div>
		<a class="item" :class="{ select: route.path === '/home' }" @click="pushSession">
			<svg class="icon">
				<use xlink:href="#icon-message"></use>
			</svg>
			<p class="unreadMsg"></p>
		</a>
		<a class="item" :class="{ select: route.path === '/relation' }" @click="pushRelation">
			<svg class="icon">
				<use xlink:href="#icon-contact"></use>
			</svg>
			<p class="unreadNotice" v-if="false"></p>
		</a>
		<a class="switch" @click="switchTheme">
			<svg class="icon">
				<use :xlink:href="theme ? '#icon-baitian_' : '#icon-yueliang'"></use>
			</svg>
		</a>
		<a class="setting" :class="{ select: route.path === '/setting' }" @click="pushSetting">
			<svg class="icon">
				<use xlink:href="#icon-setting"></use>
			</svg>
		</a>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
const route = useRoute()
const router = useRouter()
const pushSession = () => {
	router.push('/home')
}
const pushSetting = () => {
	router.push('/setting')
}
const pushRelation = () => {
	router.push('/relation')
}
const pushSelf = () => {
	router.push('/self')
}
const theme = ref(false)
const app = document.querySelector('#app') as HTMLElement
const switchTheme = () => {
	if (theme.value) {
		app.classList.remove('light')
		theme.value = false
	} else {
		app.classList.add('light')
		theme.value = true
	}
}
</script>

<style scoped>
.aside {
	display: flex;
	flex-direction: column;
	position: relative;
	align-items: center;
	height: 100%;
	background-color: var(--primary-color);
	width: 60px;
	border-radius: 8px;
	box-shadow:
		rgba(50, 50, 93, 0.25) 0px 6px 12px -2px,
		rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
	transition: all 0.8s;
	.avatar {
		margin-top: 30px;
		margin-bottom: 10px;
		width: 30px;
		height: 30px;
		border-radius: 50%;
		overflow: hidden;

		img {
			height: 100%;
			width: 100%;
		}
	}

	.icon {
		margin-top: 20px;
		width: 20px;
		height: 20px;
		fill: var(--icon-color);
	}

	.item {
		position: relative;
	}

	.unreadMsg {
		position: absolute;
		text-align: center;
		font-size: 10px;
		width: 20px;
		top: 14px;
		left: 13px;
		color: #fff;
		background-color: #fa5151;
		border-radius: 5px;
	}

	.unreadNotice {
		position: absolute;
		width: 6px;
		height: 6px;
		top: 18px;
		left: 16px;
		border-radius: 50%;
		background-color: #fa5151;
	}

	p {
		transition: 0.5s;
	}

	.select .icon {
		transform: scale(1.2);
		transition: 0.5s;
		fill: #0c68d2;
		filter: drop-shadow(0 0 1px #fa5151);
	}

	.switch {
		position: absolute;
		bottom: 60px;
		transition: all 0.5s;
	}

	.setting {
		position: absolute;
		bottom: 20px;
	}
}
</style>
