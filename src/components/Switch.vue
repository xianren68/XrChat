<template>
	<div class="switch">
		<div class="item" v-for="value in values" :key="value" :class="{ 'switch-active': value === select }" @click="handleClick(value)">
			{{ value }}
		</div>
	</div>
</template>

<script setup lang="ts">
// all the props that the switch component will take
const props = defineProps<{
	values: string[]
	select: string
}>()

// emits the selected value
const emits = defineEmits<{
	(e: 'selected', value: string): void
}>()

// when the user clicks on the switch, it will emit the selected value
const handleClick = (value: string) => {
	emits('selected', value)
}
</script>

<style scoped>
.switch {
	margin-top: 20px;
	box-sizing: border-box;
	height: 30px;
	display: flex;
	align-items: center;
	justify-content: space-around;
	border-bottom: 1px solid var(--primary-color);
	.item {
		cursor: pointer;
		padding: 0 10px;
		height: 100%;
		line-height: 30px;
		transition: all 0.3s;
	}
	.switch-active {
		color: var(--text-color);
		font-weight: 600;
		border-bottom: 2px solid #007aff;
	}
}
</style>
