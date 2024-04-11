import Request from './req.ts'
import {user} from '@/pb/user.ts'
const BaseReq = new Request("http://localhost:8080")
const userReq = BaseReq.Group('/user')

// user login.
export const Login = (data:user.LoginRequest) => userReq.Post('/login',data)

// user register.
export const Register = (data:user.RegisterRequest) => userReq.Post('/register',data)

// send email address for verify.
export const VerifyEmail = (data:user.EmailVerifyRequest) => userReq.Post('/verifyEmail',data)

// send verification code.
export const VerifyEmailCode = (data:user.EmailVerifyCode) => userReq.Post('/verifyEmailCode',data)
