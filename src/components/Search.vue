<template>
	<div class="search">
		<input
			type="text"
			:value="modelValue"
			@input="
				(e) => {
					emits('update:modelValue', (e.target as HTMLInputElement).value)
				}
			"
		/>
		<svg class="icon" @click="emits('search')" v-if="search">
			<use xlink:href="#icon-sousuo"></use>
		</svg>
		<svg class="icon" @click="emits('cancel')" v-else>
			<use xlink:href="#icon-quxiao"></use>
		</svg>
	</div>
</template>

<script setup lang="ts">
defineProps({
	modelValue: String,
	search: Boolean,
})
const emits = defineEmits(['update:modelValue', 'search', 'cancel'])
</script>

<style scoped>
.search {
	display: flex;
	position: relative;
	box-sizing: border-box;
	input {
		box-sizing: border-box;
		width: 100%;
		height: 30px;
		background-color: transparent;
		padding-left: 10px;
		padding-right: 20px;
		outline: none;
		border-radius: 5px;
		border: 1px solid #999;
		color: var(--text-color);
		border: 1px solid var(--primary-color);
	}
	.icon {
		position: absolute;
		right: 10px;
		top: 50%;
		transform: translateY(-50%);
		fill: var(--primary-color);
		width: 20px;
		height: 20px;
	}
}
</style>
