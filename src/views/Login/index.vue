<template>
	<div class="login">
		<p class="title">LOGIN</p>
		<div class="info">
			<div class="name item">
				<svg class="icon">
					<use xlink:href="#icon-user"></use>
				</svg>
				<input type="text" placeholder="邮箱" v-model="name" />
				<span class="verify" v-show="verName">账号不能为空</span>
			</div>
			<div class="pwd item">
				<svg class="icon">
					<use xlink:href="#icon-pwd"></use>
				</svg>
				<input type="password" placeholder="密码" v-model="pwd" />
				<span class="verify" v-show="verPwd">密码不能为空</span>
			</div>
			<button @click="login">LOGIN</button>
			<div class="register">
				<span>没有账号？ </span>
				<a class="link" @click="router.push({ name: 'register' })">去注册</a>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref } from 'vue'
import { Login } from '@/api'
import { user } from '@/pb/user.ts'
import { message } from '@/pb/message.ts'
import { ElMessage } from 'element-plus'
import { userStore, sessionStore } from '@/store'
import type { Session } from '@/type'
import { invoke } from '@tauri-apps/api'
import { emit } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
const router = useRouter()
const useUserStore = userStore()
const useSessionStore = sessionStore()

const name = ref('')
const pwd = ref('')
const verPwd = ref(false)
const verName = ref(false)

// login
const login = async () => {
	if (!verify()) {
		return
	}
	const arrayBuffer = await Login(
		new user.LoginRequest({
			email: name.value,
			password: pwd.value,
		})
	)
	const resp = user.LoginResponse.deserialize(arrayBuffer as Uint8Array)
	if (resp.code !== 200) {
		ElMessage.error(resp.message)
		return
	}
	const paylod = new message.Message({
		src: resp.id,
		tar: resp.id,
		msg: '',
	})
	appWindow.listen('login_result', (_e: any) => {
		// emit send online message.
		appWindow.emit('send', { message_id: 1, data: [...paylod.serializeBinary()] })
	})
	// emit tcp connect.
	emit('login', resp.id)
	useUserStore.userInfo = {
		id: resp.id,
		username: resp.username,
		line: resp.line,
		email: resp.email,
		avatar: resp.avatar,
		phone: resp.phone,
		gender: resp.gender,
	}
	useUserStore.token = resp.token
	const res = await invoke('get_session_list', { id: resp.id })
	useSessionStore.sessionList = res as Array<Session>
	router.push({ name: 'session' })
	ElMessage.success('登录成功')
}
// verify input.
const verify = () => {
	let flag = false
	if (!name) {
		verName.value = true
		setTimeout(() => {
			verName.value = false
		}, 1000)
	}
	if (!pwd.value) {
		verPwd.value = true
		setTimeout(() => {
			verPwd.value = false
		}, 1000)
	}
	flag = true
	return flag
}
</script>

<style scoped>
.login {
	margin: auto;
	height: 330px;
	width: 340px;
	background-color: #ffffff11;
	box-shadow:
		rgba(0, 0, 0, 0.1) 0px 10px 15px -3px,
		rgba(0, 0, 0, 0.05) 0px 4px 6px -2px;
	border-radius: 10px;

	.title {
		margin-top: 20px;
		font-size: 20px;
		text-align: center;
		font-weight: bolder;
	}

	.info {
		display: flex;
		flex-direction: column;
		align-items: center;
		line-height: 16px;

		.item {
			position: relative;
			border-bottom: #ccc 2px solid;
			height: 30px;
			width: 280px;

			.icon {
				display: inline-block;
				width: 20px;
				height: 20px;
			}
		}

		.name {
			margin-top: 60px;
		}

		.pwd {
			margin-top: 35px;
		}

		.email {
			margin-top: 35px;
			position: relative;

			.vef {
				position: absolute;
				right: 0;
				height: 30px;
				width: 70px;
				font-size: 12px;
				line-height: 30px;
				background-color: #ffffcf08;
				text-align: center;
			}
		}

		input {
			display: inline-block;
			height: 30px;
			width: 160px;
			margin-left: 10px;
			border: none;
			outline: none;
			background-color: transparent;
			padding: 0 0 10px 10px;
		}

		button {
			margin-top: 20px;
			background-color: #0090fc;
			font-weight: bolder;
			border-radius: 8px;
			box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
			width: 200px;
			height: 40px;
			color: #fff;
			border: none;
		}

		.register {
			margin-top: 23px;
			font-size: 12px;
			color: #ccc;

			.link {
				color: #8cc63e;
			}
		}

		.verify {
			position: absolute;
			display: inline-block;
			width: 200px;
			color: red;
			font-size: 10px;
			bottom: -20px;
			left: 10px;
		}
	}
}
</style>
