import { relation } from '@/pb/relation.ts'
import Request from './req.ts'
import { user } from '@/pb/user.ts'
import { option } from '@/pb/option.ts'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'
import { userStore } from '@/store'
const useUserStore = userStore()
const router = useRouter()
const BaseReq = new Request('http://localhost:8080')

const userReq = BaseReq.Group('/user')
const AuthReq = BaseReq.Group('/auth')
// request interceptors,add token.
AuthReq.setInterceptors('request', <T extends RequestInit | Response>(config: T): T => {
	const token = useUserStore.token
	;(config.headers as Headers).set('Authorization', 'Bearer ' + token)
	return config
})

AuthReq.setInterceptors('response', <T extends RequestInit | Response>(config: T): T => {
	const token = (config.headers as Headers).get('Authorization')
	if (token === 'Bearer ' + 'true') {
		return config
	}
	ElMessage.error('token error')
	router.push('/login')
	throw new Error('token error')
})
const optionReq = AuthReq.Group('/option')
const relationReq = AuthReq.Group('/relation')

/// user req
// user login.
export const Login = (data: user.LoginRequest) => userReq.Post('/login', data)

// user register.
export const Register = (data: user.RegisterRequest) => userReq.Post('/register', data)

// send email address for verify.
export const VerifyEmail = (data: user.EmailVerifyRequest) => userReq.Post('/verifyEmail', data)

// send verification code.
export const VerifyEmailCode = (data: user.EmailVerifyCode) => userReq.Post('/verifyEmailCode', data)

/// relation req
export const AddFriendReq = (data: relation.AddFriendRequest) => relationReq.Post('/addFriendReq', data)

// auth req
export const updateName = (data: option.updateName) => optionReq.Post('/updateName', data)

export const updateGender = (data: option.updateGender) => optionReq.Post('/updateGender', data)

export const updateLine = (data: option.updateLine) => optionReq.Post('/updateLine', data)

export const updatePhone = (data: option.updatePhone) => optionReq.Post('/updatePhone', data)
