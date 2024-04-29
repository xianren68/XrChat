export type UserInfo = {
	username: string
	id: number
	line: string
	email: string
	avatar: string
	phone: string
	gender: boolean
}

export type Session = {
	id: number
	avatar: string
	last: string
	last_time: number
	un_read: number
	name: string
	remark: string
	session_type: number
}

export type Paylod = {
	message_id: number
	data: Array<number>
}
