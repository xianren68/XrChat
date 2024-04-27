<template>
	<div class="self">
		<div class="main">
			<div class="avatar">
				<img src="" alt="" />
			</div>
			<input type="text" class="name" :placeholder="useUserStore.userInfo!.username" v-if="editName" @keydown.enter="editNameFn" />
			<div class="name" v-else @dbclick="editName = true">{{ useUserStore.userInfo!.username }}</div>
			<input
				type="text"
				class="line"
				:placeholder="useUserStore.userInfo!.line === '' ? '请输入个性签名' : ''"
				v-if="editLine"
				@keydown.enter="editLineFn"
				:value="useUserStore.userInfo!.line"
			/>
			<div class="line" @dblclick="editLine = true" v-else>{{ useUserStore.userInfo!.line === '' ? '暂未设置' : useUserStore.userInfo!.line }}</div>
			<div class="phone">
				<svg class="icon">
					<use xlink:href="#icon-shouji"></use>
				</svg>
				<input
					type="text"
					:placeholder="useUserStore.userInfo!.phone === '' ? '请输入手机号' : ''"
					v-if="editPhone"
					@keydown.enter="editPhoneFn"
					:value="useUserStore.userInfo!.phone"
				/>
				<div @dblclick="editPhone = true" v-else>{{ useUserStore.userInfo!.phone === '' ? '暂未设置' : useUserStore.userInfo!.phone }}</div>
			</div>
			<div class="gender">
				<svg class="icon">
					<use xlink:href="#icon-xingbie"></use>
				</svg>
				<div class="select">
					<input type="radio" name="gender" id="male" value="male" @change="editGenderFn" v-model="gender" />
					<label for="male">男</label>
				</div>
				<div class="select">
					<input type="radio" name="gender" id="female" value="female" @change="editGenderFn" v-model="gender" />
					<label for="female">女</label>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { updateName, updateGender, updateLine, updatePhone } from '@/api'
import { verifyPhone } from '@/utils'
import { userStore } from '@/store'
import { option } from '@/pb/option'
import { ElMessage } from 'element-plus'
import { user } from '@/pb/user'
const useUserStore = userStore()
const editName = ref(false)
const editPhone = ref(false)
const editLine = ref(false)
const gender = ref(useUserStore.userInfo!.gender ? 'male' : 'female')
console.log(gender.value)
// send editname request.
const editNameFn = async (e: Event) => {
	const value = (e.target as HTMLInputElement).value
	if (useUserStore.userInfo!.username === value) {
		editName.value = false
		return
	}
	const reqInfo = new option.updateName({
		id: useUserStore.userInfo?.id,
		name: value,
	})
	try {
		const resp = await updateName(reqInfo)
		const res = user.Response.deserialize(resp as Uint8Array)
		if (res.code !== 200) {
			ElMessage.error(res.message)
			editName.value = false
			return
		}
		useUserStore.userInfo!.username = value
		ElMessage.success('修改成功')
	} catch (e) {
		console.log(e)
	}
	editName.value = false
}
const editLineFn = async (e: Event) => {
	const value = (e.target as HTMLInputElement).value
	if (useUserStore.userInfo!.line === value) {
		editLine.value = false
		return
	}
	const reqInfo = new option.updateLine({
		id: useUserStore.userInfo?.id,
		line: value,
	})
	try {
		const resp = await updateLine(reqInfo)
		const res = user.Response.deserialize(resp as Uint8Array)
		if (res.code !== 200) {
			ElMessage.error(res.message)
			editLine.value = false
			return
		}
		useUserStore.userInfo!.line = value
		ElMessage.success('修改成功')
	} catch (e) {
		console.log(e)
	}
	editLine.value = false
}
const editPhoneFn = async (e: Event) => {
	const value = (e.target as HTMLInputElement).value
	if (!verifyPhone(value)) {
		ElMessage.warning('请输入正确的手机号')
		editPhone.value = false
		return
	}
	if (useUserStore.userInfo!.phone === value) {
		editPhone.value = false
		return
	}
	if (useUserStore.userInfo!.phone === '') {
		editPhone.value = false
		return
	}
	const reqInfo = new option.updatePhone({
		id: useUserStore.userInfo?.id,
		phone: value,
	})
	try {
		const resp = await updatePhone(reqInfo)
		const res = user.Response.deserialize(resp as Uint8Array)
		if (res.code !== 200) {
			ElMessage.error(res.message)
			editPhone.value = false
			return
		}
		useUserStore.userInfo!.phone = value
		console.log(value)
		ElMessage.success('修改成功')
	} catch (e) {
		console.log(e)
	}
	editPhone.value = false
}
// send editGender request.
const editGenderFn = async (e: Event) => {
	console.log(useUserStore.userInfo!.gender)
	const value = (e.target as HTMLInputElement).value === 'male' ? true : false
	if (useUserStore.userInfo!.gender === value) {
		return
	}
	const reqInfo = new option.updateGender({
		id: useUserStore.userInfo?.id,
		gender: value,
	})
	try {
		const resp = await updateGender(reqInfo)
		const res = user.Response.deserialize(resp as Uint8Array)
		if (res.code !== 200) {
			ElMessage.error(res.message)
			return
		}
		useUserStore.userInfo!.gender = value
		ElMessage.success('修改成功')
	} catch (e) {
		console.log(e)
	}
}
</script>

<style scoped>
.self {
	height: 100%;
	width: calc(100% - 60px);
	display: flex;
	justify-content: center;
	align-items: center;

	.main {
		box-sizing: border-box;
		position: relative;
		height: 400px;
		width: 600px;
		background-color: var(--primary-color);
		border-radius: 5px;
		box-shadow: 4px 4px 4px rgba(0, 0, 0, 0.2);
		display: flex;
		flex-direction: column;
		align-items: center;

		input[type='text'] {
			background-color: transparent;
			width: 150px;
			border: none;
			outline: none;
			border-bottom: 1px solid var(--text-color-secondary);
			color: var(--text-color);
		}

		.avatar {
			position: absolute;
			height: 100px;
			width: 100px;
			top: -50px;
			left: 50%;
			transform: translateX(-50%);
			border-radius: 50%;
			box-shadow: 0px 0px 4px rgba(0, 0, 0, 0.2);
			overflow: hidden;

			img {
				height: 100%;
				width: 100%;
			}
		}

		.name {
			margin-top: 80px;
			height: 14px;
			line-height: 14px;
			text-align: center;
			color: var(--text-color);
		}

		.line {
			margin-top: 20px;
			height: 30px;
			font-size: 20px;
			line-height: 30px;
			font-weight: 600;
			font-style: italic;
			color: var(--text-color-secondary);
		}

		.icon {
			height: 30px;
			width: 30px;
			fill: var(--text-color-secondary);
			margin-right: 10px;
		}

		.phone {
			display: flex;
			align-items: center;
			margin-top: 20px;
			color: var(--text-color);
			min-width: 135px;
			div {
				flex: 1;
				height: 30px;
				line-height: 30px;
			}
		}

		.gender {
			display: flex;
			align-items: center;
			margin-top: 20px;
			color: var(--text-color);

			.select {
				margin-left: 10px;
				display: flex;
			}
		}
	}
}
</style>
