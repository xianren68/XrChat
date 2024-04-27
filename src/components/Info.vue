<template>
	<div class="info">
		<div class="avatar">
			<img :src="avatar" alt="" />
		</div>
		<div class="content">
			<div class="name">{{ name }}</div>
			<div class="msg">{{ msg }}</div>
		</div>
		<!-- unread and last_time -->
		<slot></slot>
	</div>
</template>

<script setup lang="ts">
defineProps({
	avatar: String,
	msg: String,
	name: String,
})
</script>

<style scoped>
.info {
	margin-top: 10px;
	box-sizing: border-box;
	border-radius: 5px;
	height: 60px;
	width: 100%;
	display: flex;
	align-items: center;
	background-color: var(--primary-color);
	padding: 10px;
	transition: all 0.8s;
	.avatar {
		height: 40px;
		width: 40px;
		border-radius: 50%;
		overflow: hidden;
		margin: 0 10px;
		img {
			height: 100%;
			width: 100%;
		}
	}
	.content {
		margin-left: 10px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		height: 40px;
		width: 100px;
		margin-right: 20px;
		.name {
			font-size: 16px;
			overflow: hidden;
			white-space: nowrap;
			text-overflow: ellipsis;
			color: var(--text-color);
		}
		.msg {
			font-size: 14px;
			overflow: hidden;
			white-space: nowrap;
			text-overflow: ellipsis;
			color: var(--text-color-secondary);
		}
	}
}
</style>
