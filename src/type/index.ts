import {user} from '@/pb/user'

export type UserInfo = Omit<user.LoginResponse, 'code' | 'message' | 'token'>